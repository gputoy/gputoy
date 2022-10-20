import vars from '$lib/vars'
import { get, writable } from 'svelte/store'
import type { Config, Files, Layout, Project, ProjectResponse, ProjectUpsert } from '../generated/types'

/**
 * STATIC DEFAULTS
 */
const DEFAULT_LAYOUT: Layout = {
    isStatusOpen: true,
    fileIndex: 0,
    workspace: ["shaders/main.wgsl", "run.json"] as string[]
} as const
const DEFAULT_CONFIG: Config = {} as const
const DEFAULT_FILES: Files = {
    map: {
        "shaders/main.wgsl": {
            "data": "...",
            "dir": "shaders",
            "fileName": "main",
            "extension": "wgsl",
        },
        "run.json": {
            "data": "...",
            "dir": "",
            "fileName": "run",
            "extension": "json"
        }
    }
} as const


export type ProjectMeta = {
    title: string,
    description?: string,
}

export const wProjectId = writable<string | undefined>(undefined)

export const wFiles = {
    ...writable<Files>(DEFAULT_FILES),
    // writeFile: (fileid: string, data: string) => {
    //     const map = self.map
    //     console.log(map)
    // }
}
export const wLayout = writable<Layout>(DEFAULT_LAYOUT)
export const wConfig = writable<Config>(DEFAULT_CONFIG)

export const wProjectMeta = writable<ProjectMeta>({
    title: "New Project",
    description: "Hello to new project!",
})

/**
 * 
 * @returns Non-reactive get for project in store memory
 */
export function getProject(): Project {
    return {
        files: get(wFiles),
        layout: null,
        config: get(wConfig),
    }
}

/**
 * 
 * @param projectId Loads project from api or local host
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
    const { id, files, layout, config, title, authorId, description, published } = project as ProjectResponse

    wProjectId.set(id)
    wFiles.set(files)
    wConfig.set(config ?? DEFAULT_CONFIG)
    wLayout.set(layout ?? DEFAULT_LAYOUT)
    wProjectMeta.set({ title, description: description ?? undefined })
}

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
    }

}



