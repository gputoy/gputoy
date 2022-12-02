import type { Action, CompileError, Config, FileDependencyInfo, FilePrebuildResult, Files, Layout, PrebuildResult, ProjectResponse, UserEditorPrefs, UserGeneralPrefs, UserInfoResponse, UserPrefs } from "$common"
import { initConsoleMethods, type ConsoleExtras, type Log } from "$core/console"
import { DEFAULT_CONFIG, DEFAULT_FILES, DEFAULT_LAYOUT, DEFAULT_USER_EDITOR_PREFS, DEFAULT_USER_GENERAL_PREFS, DEFAULT_USER_KEYBINDS, type MenuKey } from "$core/consts"
import { initFilesMethods, type FilesExtras } from "$core/files"
import type { Keybinds } from "$core/input"
import { initLayoutMethods, type LayoutExtras } from "$core/layout"
import { writeToLocalStorage } from "$core/preferences"
import { writeToProjectLocalStorage, type ProjectMeta } from "$core/project"
import { initUserMethods, type UserExtras } from "$core/user"
import { initTheme, type Theme } from "$core/util"
import { derived, get, writable, type Writable } from "svelte/store"


// TODO: move this to seperate file
export type PrebuildResultExtras = {
    getFileBuild: (fileid: string) => FilePrebuildResult | null
    getFileDeps: (fileid: string) => FileDependencyInfo | null
}
export function initPrebuildResultMethods(prebuildResults: Writable<PrebuildResult | null>): PrebuildResultExtras {
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
        return get(prebuildResults)?.fileBuilds.map((fileid, val) => console.log(fileid, val))
    }
    return { getFileBuild, getFileDeps }
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
export const wPrebuildResult = makeEnhanced<PrebuildResult | null, PrebuildResultExtras>(null, initPrebuildResultMethods)()
export const wPrebuildDirty = writable(true)
export const wBuildResult = writable<{} | null>(null)
export const wBuildDirty = writable(true)

/**
 *                  Project stores parts, merges in derived dProject
 */
export const wProjectId = writable<string | null>(null)
export const wFiles = makeEnhanced<Files, FilesExtras>(DEFAULT_FILES, initFilesMethods)()
export const wConfig = makeEnhanced<Config, {}>(DEFAULT_CONFIG, () => ({}))()
export const wLayout = makeEnhanced<Layout, LayoutExtras>(DEFAULT_LAYOUT, initLayoutMethods)()
export const wProjectMeta = writable<ProjectMeta>({
    title: "New Project",
    description: "This is a new project",
    authorId: null,
    published: false,
} as ProjectMeta)

/**
 *                  Console
 */
export const wConsole = makeEnhanced<Log[], ConsoleExtras>([], initConsoleMethods)()
export const wConsoleOpen = writable(false)
export const wConsoleHistory = writable<string[]>([])
export const wConsoleHistoryIndex = writable(0)
export const wConsoleCompletionIndex = writable(0)

/**
 *                  User and auth stores
 */
export const wUser = makeEnhanced<UserInfoResponse | null, UserExtras>(null, initUserMethods)()

/**
 *                  Misc stores
 */
export const wLastInputAction = writable<{ code: string, action?: Action } | null>(null)
export const wUserModalOpen = writable(false)
export const wUserPrefsOpen = writable(false)

export const wMenuOpen = writable<Record<MenuKey, boolean>>({
    'file': false,
    'edit': false,
    'help': false,
    'view': false,
    'project': false,
})

export const wDebugPanel = writable(false)
export const wTheme = writable<Theme>("dark", initTheme)

/**
 *                  User preferences
 */
export const wUserGeneralPrefs = writable<UserGeneralPrefs>(DEFAULT_USER_GENERAL_PREFS)
export const wUserEditorPrefs = writable<UserEditorPrefs>(DEFAULT_USER_EDITOR_PREFS)
export const wUserKeybinds = writable<Keybinds>(DEFAULT_USER_KEYBINDS)
export const wUserTheme = writable<any>({})

/**
 *                  Derives stores
 */
export const dUserPrefs = derived(
    [wUserGeneralPrefs, wUserEditorPrefs, wUserKeybinds, wUserTheme],
    ([$general, $editor, $keybinds, $theme]): UserPrefs => {
        return {
            general: $general,
            editor: $editor,
            keybinds: $keybinds,
            theme: $theme,
        }
    })
dUserPrefs.subscribe(writeToLocalStorage)

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
dProject.subscribe(p => {
    if (p != null)
        writeToProjectLocalStorage(p)
})