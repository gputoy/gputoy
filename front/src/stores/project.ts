import context, { init } from '$lib/context'
import vars from '$lib/vars'
import debounce from 'lodash/debounce'
import generate from 'project-name-generator'
import { derived, get, writable } from 'svelte/store'
import { v4 } from 'uuid'
import type { Config, Layout, Project, ProjectResponse, ProjectUpsert } from '../generated/types'
import { wUser } from './auth'
import makeFiles, { DEFAULT_FILES } from './project/files'

/**
 * STATIC DEFAULTS
 */
const DEFAULT_LAYOUT: Layout = {
    isStatusOpen: true,
    fileIndex: 0,
    workspace: ["shaders/main.wgsl", "run.json"] as string[]
} as const
const DEFAULT_CONFIG: Config = {} as const


export type ProjectMeta = {
    title: string,
    description?: string,
    authorId: string | null,
    published: boolean,
    createdAt: string,
    updatedAt: string,
    forkedFromId?: string,
}

export const wProjectId = writable<string | null>(null)

export const wFiles = makeFiles()
export const wLayout = writable<Layout>(DEFAULT_LAYOUT)
export const wConfig = writable<Config>(DEFAULT_CONFIG)

export const wProjectMeta = writable<ProjectMeta>({
    title: "New Project",
    description: "This is a new project",
    authorId: null,
    published: false,
} as ProjectMeta)

/**
 * 
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
 */
export const dCanModifyProject = derived(
    [wProjectMeta, wUser],
    ([metadata, user]) => (user?.id ?? null) === metadata.authorId
)

const dProject = derived(
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

dProject.subscribe(p => {
    if (p != null)
        writeToLocalStorage(p)
})

export async function initNewProject() {
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
    const projectResponse = await fetch(vars.API_PATH + 'project/' + projectId, {
        method: 'GET',
        credentials: 'include'
    })

    const project = await projectResponse.json()
    if ("message" in project) {
        console.error("Failed to load project: ", project.message)
        return
    }
    setProject(project, true)
}

/**
 * Loads all projects belonging to user within remote storage and local storage
 */
export async function loadAllProjects(): Promise<ProjectResponse[]> {
    const userid = get(wUser)?.id ?? null
    let remoteProjects: ProjectResponse[] = []
    let localProjects: ProjectResponse[] = []
    if (userid) {
        const myProjectsResponse = await fetch(vars.API_PATH + 'project/user/' + userid, {
            method: 'GET',
            credentials: 'include'
        })
        remoteProjects = await myProjectsResponse.json()
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
        id,
        title,
        description,
        files,
        layout,
        config,
        published
    }

    const projectResponse = await fetch(vars.API_PATH + 'project', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(project)
    })

    const response = await projectResponse.json()
    if ("message" in response) {
        console.error("Something went wrong posting project: ", response.message)
        return
    }

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
    context?.free()
}


/**
 * Writes to local storage only after the alloted amount of time after 
 * last edit has occured
 * TODO: add project save timing to user config
 */
export const writeToLocalStorage = debounce(_writeToLocalStorage, 5000)
export function _writeToLocalStorage(project: ProjectResponse) {
    localStorage.setItem(project.id, JSON.stringify(project))
}

