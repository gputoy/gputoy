import { browser } from '$app/environment'
import { setCssVar } from '$core/theme'
import { PREFERENCE_KEYS, type PreferenceKey, type Preferences } from '$gen'
import DEFAULT_PREFERENCES from '$gen/prefs/defaults.json'
import debounce from 'lodash/debounce'
import _get from 'lodash/get'
import _set from 'lodash/set'
import { derived, writable, type Readable, type Writable } from 'svelte/store'

type PreferenceStore = Record<PreferenceKey, Writable<any>>

/**
 * Destructures the default preference object into a flat map.
 * @returns [key: PreferenceKey]: Writable<any>
 */
function initStore(): PreferenceStore {
	let stores = {} as PreferenceStore
	for (const key of PREFERENCE_KEYS) {
		stores[key as PreferenceKey] = writable(_get(DEFAULT_PREFERENCES, key))
	}
	return stores
}

// Internal preferences store.
// Preferences are stored by key in a flat map.
// For runtime management of preference values.
const _wPreferences = initStore()

// Derived preferences readable.
// Preferences are stored in a object with the same structure as the gpu-common struct.
// For serialization into local-storage and API calls.
export const dPreferences: Readable<Preferences> = derived(
	Object.values(_wPreferences),
	(args, set) => {
		let prefs = {} as Preferences
		for (const i in PREFERENCE_KEYS) {
			_set(prefs, PREFERENCE_KEYS[i], args[i])
		}
		set(prefs)
	}
)

/**
 * Readable preference
 * @param key key of the preference. All usable preference keys are generated to front/generated/pref-keys.ts
 * @returns A svelte readable of that preference value
 */
export function rPref(key: PreferenceKey): Readable<any> {
	return {
		subscribe: _wPreferences[key].subscribe
	}
}

/**
 * Writable preference
 * @param key key of the preference. All usable preference keys are generated to front/generated/pref-keys.ts
 * @returns A svelte writable of that preference value
 */
export function wPref(key: PreferenceKey): Writable<any> {
	return _wPreferences[key]
}

/**
 * Reset a preference to its default value
 * @param key Preference Key
 */
export function resetPreference(key: PreferenceKey) {
	_wPreferences[key].set(_get(DEFAULT_PREFERENCES, key))
}

/**
 * Return the default value of preference
 * @param key Preference key
 * @returns Default value of preference
 */
export function preferenceDefault(key: PreferenceKey): any {
	return _get(DEFAULT_PREFERENCES, key)
}

/**
 * Set multiple preferences at once.
 * @param prefs
 */
export function setPreferences(prefs?: Partial<Preferences> | null) {
	for (const key of PREFERENCE_KEYS) {
		const userValue = _get(prefs, key)
		const defaultValue = _get(DEFAULT_PREFERENCES, key)
		_wPreferences[key].set(userValue ?? defaultValue)
	}
}

/**
 * Safely convert a string to preference key
 * @param maybeKey string value that could be a key
 * @returns if key, the key, else null
 */
export function toPreferenceKey(maybeKey: string): PreferenceKey | null {
	const index = PREFERENCE_KEYS.indexOf(maybeKey.trim() as any)
	if (index < 0) return null
	return PREFERENCE_KEYS[index]
}

/**
 * Writes to local storage only after the alloted amount of time after
 * last edit has occured
 */
const writeToLocalStorage = debounce(_writeToLocalStorage, 500)
function _writeToLocalStorage(prefs: Preferences) {
	if (browser) {
		localStorage.setItem('prefs', JSON.stringify(prefs))
		localStorage.setItem('prefs:updated', JSON.stringify(Date.now()))
	}
}

dPreferences.subscribe(writeToLocalStorage)

// TODO: move to more appropirate module
_wPreferences['workspace.handle-size'].subscribe((value) =>
	setCssVar('--handle-size', value + 'px')
)
