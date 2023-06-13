import { browser } from '$app/environment'
import * as api from '$core/api'
import { DEFAULT_CONFIG, DEFAULT_FILES, DEFAULT_LAYOUT } from '$core/consts'
import type { ProjectResponse, ProjectUpsert } from '$gen'
// import context, { init } from "$core/context"
import { wConfig, wFiles, wProjectId, wProjectMeta, wUser } from '$stores'
import { toast } from '@zerodevx/svelte-toast'
import debounce from 'lodash/debounce'
import generate from 'project-name-generator'
import { get } from 'svelte/store'
import { v4 } from 'uuid'
import { getLayout, loadLayout } from './layout'

export type ProjectMeta = Omit<ProjectResponse, 'id' | 'files' | 'layout' | ''>

// {
// 	title: string
// 	description ?: string
// 	authorId: string | null
// 	published: boolean
// 	createdAt: string
// 	updatedAt: string
// 	forkedFromId ?: string
// }

/**
 * Data required to display project select ui
 */
export type ProjectSelectInfo = {
	id: string
	title: string
	saveStatus: ProjectSaveStatus
	lastUpdated: Date
}

/**
 * remote: project was fetched from remote server
 *
 * local: project was fetched from local storage
 *
 * local-changes: project was available in both remote and local,
 *                but local is more up to date
 */
export type ProjectSaveStatus = 'remote' | 'local' | 'local-changes'

/**
 * Writes to local storage only after the alloted amount of time after
 * last edit has occured
 * TODO: add project save timing to user config
 */
export const writeToProjectLocalStorage = debounce(_writeToLocalStorage, 2000)
function _writeToLocalStorage(project: ProjectResponse) {
	// TODO: doing dates like this will probably cause a problem
	if (browser) {
		project.updatedAt = Date.now().toString()
		localStorage.setItem(project.id, JSON.stringify(project))
	}
}

/**
 * Creates default project and stores within project stores
 * TODO: Init from templates instead of hardcoded default
 */
export async function initNewProject() {
	// defaults to silly two word title
	const title = generate().dashed
	const now = Date.now().toLocaleString()
	wFiles.set(DEFAULT_FILES)
	wConfig.set(DEFAULT_CONFIG)
	loadLayout(DEFAULT_LAYOUT)
	wProjectMeta.set({
		title,
		authorId: get(wUser)?.id ?? null,
		description: 'Default description',
		published: false,
		createdAt: now,
		updatedAt: now
	})
	// local projects need an id too...
	// this will be used until the user decides to upload to remote,
	// then the local storage entry should be removed
	wProjectId.set('local:' + v4())
}

/**
 * Loads project from api or local storage
 *
 * @param projectId
 * @returns
 */
export async function loadProject(projectId: string) {
	if (projectId.startsWith('local')) {
		if (!browser) return
		const project = localStorage.getItem(projectId)
		if (!project) {
			toast.push('Project not found in local storage: ' + projectId)
			return
		}
		setProject(JSON.parse(project), true)
		return
	}
	const response = await api.getProject(projectId)
	if ('message' in response) {
		// TODO: turn this awful alert into a presentable error message
		toast.push(
			`Recieved ${response.status} status on getProject response. Message: ${response.message}`
		)
		return
	}
	setProject(response, true)
}

/**
 * Loads all projects belonging to user within remote storage and local storage
 */
export async function loadAllProjects(): Promise<ProjectResponse[]> {
	const userid = get(wUser)?.id ?? null
	let remoteProjects: ProjectResponse[] = []
	let localProjects: ProjectResponse[] = []
	if (userid) {
		// TODO: switch out user projects call with call that only fetches
		// needed project info (id, title, last updated) to reduce bandwith
		const response = await api.getUserProjects(userid)
		if ('message' in response) {
			// TODO: turn this awful alert into a presentable error message
			toast.push(
				`Recieved ${response.status} status on getUserProjects response. Message: ${response.message}`
			)
		} else {
			remoteProjects = response
		}
	}
	localProjects = Object.keys(localStorage)
		.filter((k) => k.startsWith('local:'))
		.map((k) => localStorage.getItem(k))
		.filter((p) => !!p)
		.map((k) => JSON.parse(k!))
	return [...remoteProjects, ...localProjects]
}

/**
 * Saves project to remote
 * @param published
 */
export async function saveProject(published = false) {
	const id = get(wProjectId)
	const { title, description } = get(wProjectMeta)
	const files = get(wFiles)
	const config = get(wConfig)
	const layout = getLayout()

	const project: ProjectUpsert = {
		id: id?.startsWith('local') ? undefined : id,
		title,
		description,
		files,
		layout,
		config,
		published
	}

	const response = await api.updateProject(project)
	if ('message' in response) {
		// TODO: turn this awful alert into a presentable error message
		toast.push(
			`Recieved ${response.status} status on login response. Message: ${response.message}`
		)
		return
	}
	toast.push('Project saved!')
	setProject(response)
}

/**
 * Sets project in store to param
 * @param project
 */
export function setProject(project: ProjectResponse, resetContext = false) {
	const {
		id,
		files,
		layout,
		config,
		title,
		authorId,
		description,
		published,
		createdAt,
		updatedAt,
		forkedFromId
	} = project as ProjectResponse

	wProjectId.set(id)
	wFiles.set(files)
	wConfig.set(config ?? DEFAULT_CONFIG)
	loadLayout(layout ?? DEFAULT_LAYOUT)
	wProjectMeta.set({
		title,
		description: description ?? undefined,
		authorId: authorId ?? null,
		published,
		createdAt,
		updatedAt,
		forkedFromId: forkedFromId ?? undefined
	})

	if (browser) localStorage.setItem('last-project', project.id)

	if (resetContext) {
		// context?.free()
		// init()
	}
}
export function clearProject() {
	// flush any pending saves to local storage
	writeToProjectLocalStorage.flush()
	// nothing else needs to be cleared as frontend will not display editor
	// when projectId is null
	wProjectId.set(null)
	localStorage.removeItem('last-project')
	// context?.free()
}
