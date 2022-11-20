import { browser } from "$app/environment"
import { wTheme, wUserModalOpen, wUserPrefsOpen } from "$stores"
import Cookies from "js-cookie"
import type { StartStopNotifier } from "svelte/store"

export type Theme = "light" | "dark"
export const initTheme: StartStopNotifier<Theme> = (set) => {
    if (!browser) return
    const theme = Cookies.get('t')
    if (theme) {
        set(theme as Theme)
    }
}

export function toggleTheme() {
    wTheme.update((theme) => {
        if (!browser) return theme
        const newTheme = theme == 'light' ? 'dark' : 'light'
        document.body.className = newTheme
        Cookies.set('t', newTheme)
        return newTheme
    })
}

export function toggleUserModal() {
    wUserModalOpen.update(o => !o)
}
export function toggleUserPrefs() {
    wUserPrefsOpen.update(o => !o)
}

