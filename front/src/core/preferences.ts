import { browser } from '$app/environment'
import type { UserPrefs } from '$common'
import {
	DEFAULT_USER_EDITOR_PREFS,
	DEFAULT_USER_GENERAL_PREFS,
	DEFAULT_USER_KEYBINDS,
	USER_CONFIG_META,
	type ConfigKey,
	type ConfigScope,
	type GeneralConfigKey
} from '$core/consts'
import {
	wUserEditorPrefs,
	wUserGeneralPrefs,
	wUserKeybinds,
	wUserTheme
} from '$stores'
import debounce from 'lodash/debounce'

export function setUserPrefs(config?: Partial<UserPrefs>) {
	if (!config) return
	wUserGeneralPrefs.set({ ...DEFAULT_USER_GENERAL_PREFS, ...config.general })
	wUserEditorPrefs.set({ ...DEFAULT_USER_EDITOR_PREFS, ...config.editor })
	wUserKeybinds.set({ ...DEFAULT_USER_KEYBINDS, ...config.keybinds })
	wUserTheme.set(config.theme)
}

export function validate(
	scope: ConfigScope,
	configKey: ConfigKey,
	value: any
): boolean {
	// TODO: fix types to remove ts ignore
	// @ts-ignore
	const meta = USER_CONFIG_META[scope][configKey as _] as ConfigItemMeta
	switch (meta.type) {
		case 'text': {
			return meta.regex?.test(value as string) ?? true
		}
		case 'number': {
			return value >= (meta.min ?? -Infinity) && value <= (meta.max ?? Infinity)
		}
		case 'select': {
			return meta.options?.includes(value as string) ?? false
		}
		default:
			return false
	}
}

export function setProperty(
	scope: ConfigScope,
	configKey: ConfigKey,
	value: any
) {
	switch (scope) {
		case 'editor': {
			wUserEditorPrefs.update((old) => {
				// TODO: fix types to remove ts ignore
				// @ts-ignore
				old[configKey as EditorConfigKey] = value
				return old
			})
		}
		case 'general': {
			wUserGeneralPrefs.update((old) => {
				old[configKey as GeneralConfigKey] = value
				return old
			})
		}
	}
}
/**
 * Writes to local storage only after the alloted amount of time after
 * last edit has occured
 */
export const writeToLocalStorage = debounce(_writeToLocalStorage, 500)
function _writeToLocalStorage(config: UserPrefs) {
	if (browser) {
		localStorage.setItem('config', JSON.stringify(config))
		localStorage.setItem('config:updated', JSON.stringify(Date.now()))
	}
}
