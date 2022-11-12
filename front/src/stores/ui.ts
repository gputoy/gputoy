/// Store file for various UI state that doesn't need to be serialized
/// Temporary things like if the settings panel is opened, etc.

import type { MenuKey } from '$lib/dev/menu/menu'
import { writable } from 'svelte/store'

export const wUserModalOpen = writable(false)
export const wUserConfigOpen = writable(false)

export function toggleUserModal() {
    wUserModalOpen.update(o => !o)
}
export function toggleUserConfig() {
    wUserConfigOpen.update(o => !o)
}

export const wMenuOpen = writable<Record<MenuKey, boolean>>({
    'file': false,
    'edit': false,
    'help': false,
    'view': false,
    'project': false,
})

export const wDebugPanel = writable(false)