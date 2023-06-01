import { initConsoleMethods, type ConsoleExtras, type Log } from '$core/console'
import {
	DEFAULT_CONFIG,
	DEFAULT_FILES,
	DEFAULT_RUN_STATE, type MenuKey
} from '$core/consts'
import { initFilesMethods, treeFromFiles, type FilesExtras } from '$core/files'
import { dLayout } from '$core/layout'
import { writeToProjectLocalStorage, type ProjectMeta } from '$core/project'
import { initUserMethods, type UserExtras } from '$core/user'
import type {
	CompileError,
	Config,
	FileDependencyInfo,
	FilePrebuildResult,
	Files,
	PrebuildResult,
	ProjectResponse,
	UserInfoResponse
} from '$gen'
import {
	derived,
	get,
	writable,
	type Readable, type Writable
} from 'svelte/store'
import {
	initRunStateMethods,
	type RunState,
	type RunStateExtras
} from './runstate'

export function sealWritable<T>(writable: Writable<T>): Readable<T> {
	return {
		subscribe: writable.subscribe
	}
}

// TODO: move this to seperate file
export type PrebuildResultExtras = {
	getFileBuild: (fileid: string) => FilePrebuildResult | null
	getFileDeps: (fileid: string) => FileDependencyInfo | null
}
export function initPrebuildResultMethods(
	prebuildResults: Writable<PrebuildResult | null>
): PrebuildResultExtras {
	function getFileBuild(fileid: string): FilePrebuildResult | null {
		/** @ts-ignore */
		return get(prebuildResults)?.fileBuilds.get(fileid) ?? null
	}
	function getFileDeps(fileid: string): FileDependencyInfo | null {
		/** @ts-ignore */
		return get(prebuildResults)?.dependencyInfo.deps.get(fileid) ?? null
	}
	function errors(): CompileError[] | null {
		/** @ts-ignore */
		return get(prebuildResults)?.fileBuilds.map((fileid, val) =>
			console.log(fileid, val)
		)
	}
	return { getFileBuild, getFileDeps }
}

type WriableSet = Readable<{ [key: string]: boolean }> & {
	/**
	 * Add value to the set.
	 * @params val to add
	 * @returns true if this value was already in the set
	 */
	add: (val: string) => boolean
	/**
	 * Whether this value is in the set.
	 * @params val to check
	 * @returns true if this value is within the set
	 */
	has: (val: string) => boolean
	/**
	 * Remove this value from the set
	 * @params val to remove
	 * @returns true if this value is within the set
	 */
	remove: (val: string) => boolean
	/**
	 * Clear the set
	 * @returns number of values cleared
	 */
	clear: () => number
}

function makeSet(initial?: { [key: string]: boolean }): WriableSet {
	const _writable = writable(initial ?? {})
	function add(val: string): boolean {
		let exists = true
		_writable.update((set) => {
			if (set[val]) return set
			exists = false
			set[val] = true
			return set
		})
		return exists
	}
	function has(val: string): boolean {
		return get(_writable)[val]
	}
	function remove(val: string): boolean {
		let deleted = false
		_writable.update((set) => {
			if (set[val]) return set
			deleted = true
			delete set[val]
			return set
		})
		return deleted
	}
	function clear(): number {
		let count = 0
		_writable.update((set) => {
			count = Object.keys(set).length
			return {}
		})
		return count
	}
	return { add, has, remove, clear, subscribe: _writable.subscribe }
}

/**
 * Custom store. Allows store methods to be defined in seperate file from
 * the store itself. It should come from some other file within /src/core.
 */
type EnhancedWritable<Type, Extras> = Writable<Type> & Extras

/**
 * Constructor for custom store. This creates a writable and
 * passes it to the extras constructor to create the extra methods on store.
 * @param initial Initial value of store
 * @param extras Constructor for extra methods within store
 * @returns EnhancedWritable
 */
function makeEnhanced<Type, Extras>(
	initial: Type,
	extras: (w: Writable<Type>) => Extras
): () => EnhancedWritable<Type, Extras> {
	return function () {
		const _writable = writable<Type>(initial)
		return { ..._writable, ...extras(_writable) }
	}
}

/**
 *                  Core system stores
 */
export const wPrebuildResult = makeEnhanced<
	PrebuildResult | null,
	PrebuildResultExtras
>(null, initPrebuildResultMethods)()
export const wPrebuildDirty = writable(true)
export const wBuildResult = writable<{} | null>(null)
export const wBuildDirty = writable(true)
export const wRunState = makeEnhanced<RunState, RunStateExtras>(
	DEFAULT_RUN_STATE,
	initRunStateMethods
)()

/**
 *                  Project stores parts, merges in derived dProject
 */
export const wProjectId = writable<string | null>(null)

export const wFiles = makeEnhanced<Files, FilesExtras>(
	DEFAULT_FILES,
	initFilesMethods
)()
export const dFileTree = derived([wFiles], ([files]) => {
	console.log('in dFileTree', files.map)

	return treeFromFiles(files)
})
export const wModelDirty = makeSet()
export const wFileDirty = makeSet()

export const wConfig = makeEnhanced<Config, {}>(DEFAULT_CONFIG, () => ({}))()

export const wProjectMeta = writable<ProjectMeta>({
	title: 'New Project',
	description: 'This is a new project',
	authorId: null,
	published: false
} as ProjectMeta)

/**
 *                  Console
 */
export const wConsole = makeEnhanced<Log[], ConsoleExtras>(
	[],
	initConsoleMethods
)()
export const wConsoleHistory = writable<string[]>([])
export const wConsoleHistoryIndex = writable(0)
export const wConsoleCompletionIndex = writable(0)

/**
 *                  User and auth stores
 */
export const wUser = makeEnhanced<UserInfoResponse | null, UserExtras>(
	null,
	initUserMethods
)()

/**
 *                  Misc stores
 */
export const wUserRenaming = writable<string | null>(null)
export const wUserDeleting = writable<string | null>(null)

export const wMenuOpen = writable<Record<MenuKey, boolean>>({
	file: false,
	edit: false,
	help: false,
	view: false,
	project: false
})


// TODO: type this up
export const wWorkerData = writable(null)

/**
 *  Reactive var which indicates if user can modify current project
 *  If not, use will need to fork before modifying.
 */
export const dCanModifyProject = derived(
	[wProjectMeta, wUser],
	([metadata, user]) => (user?.id ?? null) === metadata.authorId
)

/**
 * Reactive var for subscribing local storage save
 */
export const dProject = derived(
	[wFiles, wConfig, dLayout, wProjectId, wProjectMeta],
	([files, config, layout, id, meta]): ProjectResponse | null => {
		if (!id) return null
		return {
			id,
			title: meta.title,
			description: meta.description,
			authorId: meta.authorId,
			files,
			config,
			layout,
			published: meta.published,
			createdAt: meta.createdAt,
			updatedAt: meta.updatedAt,
			forkedFromId: meta.forkedFromId
		}
	}
)
dProject.subscribe((p) => {
	if (p != null) writeToProjectLocalStorage(p)
})
