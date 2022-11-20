import type { Action, Config, Files, Layout, ProjectResponse, UserEditorPrefs, UserGeneralPrefs, UserInfoResponse, UserPrefs } from "$common"
import { DEFAULT_CONFIG, DEFAULT_FILES, DEFAULT_LAYOUT, DEFAULT_USER_EDITOR_PREFS, DEFAULT_USER_GENERAL_PREFS, DEFAULT_USER_KEYBINDS, type MenuKey } from "$core/consts"
import type { ProjectMeta } from "$core/project"
import { derived, writable, type Writable } from "svelte/store"

import { initFilesMethods, type FilesExtras } from "$core/files"
import type { Keybinds } from "$core/input"
import { initLayoutMethods, type LayoutExtras } from "$core/layout"
import { writeToLocalStorage } from "$core/preferences"
import { initUserMethods, type UserExtras } from "$core/user"
import { initTheme, type Theme } from "$core/util"



type EnhancedWritable<Type, Extras> = Writable<Type> & Extras

/**
 * Constructor for enhanced store. This creates a writable store and
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
 *                  User store and auth
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