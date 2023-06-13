import * as layout from '$core/layout'
import { clearProject } from '$core/project'
import type {
	Action,
	CopyMove,
	Delete,
	FilePath,
	SupportedExtension
} from '$gen'
import { getAllModels, getModel } from '$monaco'
import { wWorkerInternal } from '$monaco/wgsl/wgslMode'
import {
	getStore,
	getStores,
	wBuildDirty,
	wConfig,
	wConsole,
	wFileDirty,
	wFiles,
	wModelDirty,
	wPrebuildDirty,
	wRunState,
	wWorkerData
} from '$stores'
import { toast } from '@zerodevx/svelte-toast'
import isEqual from 'lodash/isEqual'
import { get } from 'svelte/store'
import { prettyPrintJson } from './console'
import context from './context'
import { fileWithNewPath, getChildren, pathToParts } from './files'
import { bindKey } from './keys'

/**
 * Determines if two Actions are equal by value
 * @param a
 * @param b
 * @returns true/false if actions are equal by value
 */
export function isActionEqual(a: Action, b: Action): boolean {
	if (a.ty !== b.ty) return false
	return 'c' in a && 'c' in b ? isEqual(a.c, b.c) : true
}

/**
 * Evaluates filter from filteredAction
 * TODO: implement specific filters
 * @param filter
 * @returns
 */
export function actionPermitted(fAction: { action: Action }) {
	return true
}

export const FILTER_CONDITION_LIST = [
	'userOwnsProject',
	'userLoggedIn',

	'filesDirty',
	'fileDirty',
	'fileOpen',

	'editorPanelFocused',
	'resourcePanelFocused',
	'projectPanelFocused',
	'viewportPanelFocused'
]
export type SingleFilter = (typeof FILTER_CONDITION_LIST)[number]

/**
 * Pushes Action to be executed
 * @param action Action to be executed
 */
export function pushAction(action: Action) {
	switch (action.ty) {
		case 'clear':
			clearConsole()
			break
		case 'playPause':
			playPause()
			break
		case 'openTab':
			layout.openTab(action.c)
			break
		case 'closeTab':
			layout.closeTab()
		case 'nextTab':
			layout.shiftTab(1)
			break
		case 'prevTab':
			layout.shiftTab(-1)
			break
		case 'build':
			rebuildProject()
			break
		case 'reset':
			resetProject()
			break
		case 'toggleUi':
			layout.toggleRegionVisibility(action.c)
			break
		case 'toggleAllPanes':
			layout.toggleAllPanes()
			break
		case 'closeTab':
			layout.closeTab()
			break
		case 'setRunner':
			setRunner(action.c)
			break
		case 'saveFile':
			saveCurrentFile()
			break
		case 'saveAllFiles':
			saveAllFiles()
			break
		case 'move':
			moveFile(action.c)
			break
		case 'copy':
			copyFile(action.c)
			break
		case 'delete':
			deleteFile(action.c)
			break
		case 'newFile':
			createNewFile(action.c)
			break
		case 'exit':
			exit()
			break
		case 'bindKey':
			bindKey(action.c)
			break
		case 'dump':
			dump(action.c)
			break

		/** @ts-ignore */
		// There may be a case in the future where a new variant is added
		// and the implementation was forgotten
		default:
			toast.push('Action unrecognized or not yet implemented: ' + action.ty)
	}
}

/**
 *  TODO: create action reversal system
 * @param action
 */
export function reverseAction(action: Action) {}

/// ------------------- Action execution ----------------------

async function playPause() {
	if (get(wPrebuildDirty)) {
		context.prebuild()
	}

	if (get(wBuildDirty)) {
		context.build()
	}
	wWorkerInternal.tryBuild()
	wWorkerData.set(await wWorkerInternal.poll())
	wRunState.playPause()
}

function rebuildProject() {}

function resetProject() {}

function clearConsole() {
	wConsole.set([])
}

function focusPane(c: string) {}

function exit() {
	clearProject()
}

function setRunner(fileid: string) {
	if (wFiles.getFile(fileid))
		wConfig.update((config) => {
			config.runner = fileid
			return config
		})
	else wConsole.error('Runner not found: ' + fileid)
}

async function saveCurrentFile() {
	const currentFile = layout.getOpenFileId()
	if (!currentFile) return
	await wWorkerInternal.applyUpdateToFile(currentFile)
	wModelDirty.remove(currentFile)

	const model = getModel(currentFile)
	if (!model) return
	wFiles.writeFile(currentFile, model.getValue())
	wFileDirty.remove(currentFile)
}

async function saveAllFiles() {
	await wWorkerInternal.applyUpdateToFile()
	wModelDirty.clear()

	getAllModels().forEach((model) => {
		const path = model.uri.path
		wFiles.writeFile(path, model.getValue())
	})
	wFileDirty.clear()
}

function createNewFile(args: FilePath) {
	if (wFiles.getFile(args) != null) {
		wConsole.error('File already exists: ' + args)
	}
	const [fileName, extension, dirs] = pathToParts(args)!
	wFiles.newFile({
		id: args,
		fileName,
		extension: extension as SupportedExtension,
		dir: dirs.pop() ?? '',
		data: ''
	})
}

function moveFile(args: CopyMove) {
	let paths = wFiles.paths()
	if (!paths.includes(args.src)) {
		wConsole.error("Source path doesn't exist: " + args.src)
		return
	}
	if (paths.includes(args.dest)) {
		wConsole.error('Destination path already exists: ' + args.dest)
		return
	}
	const { src, dest, isDir } = args
	if (isDir) {
		const childPaths = getChildren(src)
		childPaths
			.map((p) => ({ src: p, dest: p.replace(src, dest), isDir: false }))
			.forEach((arg) => moveFile(arg))
		layout.moveFileTreeState(src, dest)
		return
	}
	let didUpdate = false
	wFiles.update(({ map }) => {
		let curr = map[src]
		if (curr && map[dest] == undefined) {
			const newFile = fileWithNewPath(curr, dest)
			if (!newFile) return { map }
			map[dest] = newFile
			delete map[src]
			didUpdate = true
		}
		return { map }
	})
	if (didUpdate) {
		layout.replaceIdInTabs(src, dest)
	}
}

function copyFile(args: CopyMove) {
	throw new Error('Function not implemented.')
}

var _undoDeleteFiles = []

// TODO: scrap recursive implementation for one that gathers the
// list of deleted file ids before-hand.
function deleteFile(args: Delete) {
	const { path, isDir } = args

	if (isDir) {
		const childPaths = getChildren(path)
		childPaths.forEach((p) => deleteFile({ path: p, isDir: false }))
		return
	}
	let deletedFile = wFiles.removeFile(path)
	if (deletedFile) {
		layout.deleteIdInTabs(path)
	}
}

function dump(storeKey: string) {
	storeKey = storeKey.slice(1)
	let value
	if (storeKey == '*') {
		value = getStores()
	} else {
		let store = getStore(storeKey)
		if (!store) {
			wConsole.error('Invalid store-key: ' + storeKey)
			return
		}
		value = get(store)
	}
	let json = prettyPrintJson(value)
	wConsole.out(json)
}
