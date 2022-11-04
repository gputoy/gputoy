import { DEFAULT_USER_EDITOR_CONFIG, DEFAULT_USER_GENERAL_CONFIG, DEFAULT_USER_KEYBINDS, USER_CONFIG_META, type ConfigItemMeta, type ConfigKey, type ConfigScope, type GeneralConfigKey } from "$lib/consts/userConfig"
import type { FilteredAction, UserConfig, UserEditorConfig, UserGeneralConfig } from "src/generated/types"
import { derived, writable } from "svelte/store"

export const wUserGeneralConfig = writable<UserGeneralConfig>(DEFAULT_USER_GENERAL_CONFIG)
export const wUserEditorConfig = writable<UserEditorConfig>(DEFAULT_USER_EDITOR_CONFIG)
export const wUserKeybinds = writable<{ [key: string]: FilteredAction }>(DEFAULT_USER_KEYBINDS)
export const wUserTheme = writable<any>({})
export const dUserConfig = derived(
    [wUserGeneralConfig, wUserEditorConfig, wUserKeybinds, wUserTheme],
    ([$general, $editor, $keybinds, $theme]): UserConfig => {
        return {
            general: $general,
            editor: $editor,
            keybinds: $keybinds,
            theme: $theme,
        }
    })

export function setUserConfig(config: Partial<UserConfig>) {
    wUserGeneralConfig.set({ ...DEFAULT_USER_GENERAL_CONFIG, ...config.general })
    wUserEditorConfig.set({ ...DEFAULT_USER_EDITOR_CONFIG, ...config.editor })
    wUserKeybinds.set({ ...DEFAULT_USER_KEYBINDS, ...config.keybinds })
    wUserTheme.set(config.theme)
}

export function validate(scope: ConfigScope, configKey: ConfigKey, value: any): boolean {
    // TODO: fix types to remove ts ignore
    // @ts-ignore
    const meta = USER_CONFIG_META[scope][configKey as _] as ConfigItemMeta
    switch (meta.type) {
        case 'text': {
            return meta.regex?.test(value as string) ?? true
        }
        case 'number': {
            return value >= (meta.min ?? -Infinity)
                && value <= (meta.max ?? Infinity)
        }
        case 'select': {
            return meta.options?.includes(value as string) ?? false
        }
        default: return false
    }
}

export function setProperty(scope: ConfigScope, configKey: ConfigKey, value: any) {
    switch (scope) {
        case 'editor': {
            wUserEditorConfig.update(old => {
                // TODO: fix types to remove ts ignore
                // @ts-ignore
                old[configKey as EditorConfigKey] = value
                return old
            })
        }
        case 'general': {
            wUserGeneralConfig.update(old => {
                old[configKey as GeneralConfigKey] = value
                return old
            })
        }
    }
}