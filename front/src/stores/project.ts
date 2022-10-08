import vars from '$lib/vars';
import { get, writable } from 'svelte/store';

const DEFAULT_LAYOUT: Layout = {} as const;
const DEFAULT_CONFIG: Config = {} as const;

type Project = {
    id: string,
    title: string,
    description?: string,
    files: Files,
    layout?: Layout,
    config?: Config,
    published: boolean,
    createdAt: Date,
    updatedAt: Date,
    authorId?: string,
    forkedFromId?: string,
};

type ProjectSave = {
    id?: string,
    title: string,
    description?: string,
    files: Files,
    layout?: Layout,
    config?: Config,
    published: boolean,
}

export type ProjectMeta = {
    title: string,
    description?: string,
}

export type Files = {

};

export type Layout = {

};

export type Config = {

};

export const wProjectId = writable<string | undefined>(undefined);

export const wFiles = writable<Files>({

});

export const wLayout = writable<Layout>({

});

export const wConfig = writable<Config>({

});

export const wProjectMeta = writable<ProjectMeta>({
    title: "New Project",
    description: "Hello to new project!",
})

export async function loadProject(projectId: string) {
    const projectResponse = await fetch(vars.API_PATH + 'project/' + projectId, {
        method: 'GET',
        credentials: 'include'
    });

    const project = await projectResponse.json();
    if ("message" in project) {
        console.error("Failed to load project: ", project.message);
        return
    }
    const { id, files, layout, config } = project as Project;

    wProjectId.set(id);
    wFiles.set(files);
    wConfig.set(config ?? DEFAULT_CONFIG);
    wLayout.set(layout ?? DEFAULT_LAYOUT);
}

export async function saveProject(published: boolean = false) {
    const id = get(wProjectId);
    const { title, description } = get(wProjectMeta);
    const files = get(wFiles);
    const config = get(wConfig);
    const layout = get(wLayout);

    const project: ProjectSave = {
        id,
        title,
        description,
        files,
        layout,
        config,
        published
    };

    const projectResponse = await fetch(vars.API_PATH + 'project', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(project)
    });

    const response = await projectResponse.json();
    if ("message" in response) {
        console.error("Something went wrong posting project: ", response.message);
    }

}



