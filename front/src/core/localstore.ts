import { browser } from "$app/environment"
import debounce from "lodash/debounce"
import type { Readable } from "svelte/store"

/**
 * Bind a readable to localstore.
 * @param readable A svelte readable
 * @param storeStrategy Either a string key or a record of keys to json derived from store value
 * @param debounceDuration The duration to wait after the last update before writing to local storage.
 * @returns 
 */
export function bind<T>(readable: Readable<T>, storeStrategy: string | ((value: T) => [string, any][]), debounceDuration = 500) {
    if (!browser) return

    let callback: (value: T) => void
    if (typeof storeStrategy == 'string') {
        callback = (value: T) => localStorage.setItem(storeStrategy, JSON.stringify(value))
    } else {
        callback = (value: T) => {
            const storeValues = storeStrategy(value)
            for (let [key, value] of storeValues) {
                localStorage.setItem(key, JSON.stringify(value))
            }
        }
    }

    setTimeout(() => readable.subscribe(debounce(callback, debounceDuration)), 100)
}

export function get<T>(key: string): T | null {
    if (!browser) return null
    const rawValue = localStorage.getItem(key)
    if (!rawValue) return null
    return JSON.parse(rawValue) as T ?? null
}