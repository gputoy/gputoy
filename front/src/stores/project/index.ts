import { browser } from '$app/environment'
import { DEFAULT_CONFIG, DEFAULT_FILES, DEFAULT_LAYOUT } from '$lib/consts/project'
import * as api from '$lib/core/api'
import context, { init } from '$lib/core/context'
import { toast } from '@zerodevx/svelte-toast'
import debounce from 'lodash/debounce'
import generate from 'project-name-generator'
import type { Project, ProjectResponse, ProjectUpsert } from 'src/generated/types'
import { derived, get, writable } from 'svelte/store'
import { v4 } from 'uuid'
import { wUser } from '../auth'
import makeConfig from './config'
import makeFiles from './files'
import makeLayout from './layout'

/**
 * Project metadata, basically everything besides files, layout, and config
 * This is split so each of these stores of components of the project can be
 * sunscribed to seperately
 * TODO: maybe include this within gpu-core?
 */
export type ProjectMeta = {
    title: string,
    description?: string,
    authorId: string | null,
    published: boolean,
    createdAt: string,
    updatedAt: string,
    forkedFromId?: string,
}

/**
 * Data required to display project select ui
 */
export type ProjectSelectInfo = {
    id: string,
    title: string,
    saveStatus: ProjectSaveStatus,
    lastUpdated: Date,
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

// Main project stores 
export const wProjectId = writable<string | null>(null)
export const wFiles = makeFiles()
export const wLayout = makeLayout()
export const wConfig = makeConfig()
export const wProjectMeta = writable<ProjectMeta>({
    title: "New Project",
    description: "This is a new project",
    authorId: null,
    published: false,
} as ProjectMeta)

/**
 * @returns Non-reactive get for project in store memory
 */
export function getProject(): Project {
    return {
        files: get(wFiles),
        layout: get(wLayout),
        config: get(wConfig),
    }
}

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
    [wFiles, wConfig, wLayout, wProjectId, wProjectMeta],
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
            forkedFromId: meta.forkedFromId,

        }
    }
)

/**
 * Writes to local storage only after the alloted amount of time after 
 * last edit has occured
 * TODO: add project save timing to user config
 */
const writeToLocalStorage = debounce(_writeToLocalStorage, 500)
function _writeToLocalStorage(project: ProjectResponse) {
    // TODO: doing dates like this will probably cause a problem
    if (browser) {
        project.updatedAt = Date.now().toString()
        localStorage.setItem(project.id, JSON.stringify(project))
    }
}
dProject.subscribe(p => {
    if (p != null)
        writeToLocalStorage(p)
})

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
    wLayout.set(DEFAULT_LAYOUT)
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
    await init()
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
    }
    const response = await api.getProject(projectId)
    if ('message' in response) {
        // TODO: turn this awful alert into a presentable error message
        toast.push(`Recieved ${response.status} status on getProject response. Message: ${response.message}`)
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
            toast.push(`Recieved ${response.status} status on getUserProjects response. Message: ${response.message}`)
        } else {
            remoteProjects = response
        }
    }
    localProjects = Object.keys(localStorage)
        .filter(k => k.startsWith("local:"))
        .map(k => localStorage.getItem(k))
        .filter(p => !!p)
        .map(k => JSON.parse(k!))
    return [...remoteProjects, ...localProjects]
}

/**
 * Saves project to remote 
 * @param published 
 */
export async function saveProject(published: boolean = false) {
    const id = get(wProjectId)
    const { title, description } = get(wProjectMeta)
    const files = get(wFiles)
    const config = get(wConfig)
    const layout = get(wLayout)

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
        toast.push(`Recieved ${response.status} status on login response. Message: ${response.message}`)
        return
    }
    toast.push('Project saved!')
    setProject(response)
}

/**
 * Sets project in store to param
 * @param project 
 */
export function setProject(project: ProjectResponse, resetContext: boolean = false) {
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
    wLayout.set(layout ?? DEFAULT_LAYOUT)
    wProjectMeta.set({
        title,
        description: description ?? undefined,
        authorId: authorId ?? null,
        published,
        createdAt,
        updatedAt,
        forkedFromId: forkedFromId ?? undefined
    })

    if (browser)
        localStorage.setItem('last-project', project.id)


    if (resetContext) {
        context?.free()
        init()
    }
}
export function clearProject() {
    // flush any pending saves to local storage
    writeToLocalStorage.flush()
    // nothing else needs to be cleared as frontend will not display editor
    // when projectId is null
    wProjectId.set(null)
    localStorage.removeItem('last-project')
    context?.free()
}

