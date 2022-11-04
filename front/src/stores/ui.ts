import { writable } from 'svelte/store'

export const wUserModalOpen = writable(false)
export const wUserConfigOpen = writable(false)

export function toggleUserModal() {
    wUserModalOpen.update(o => !o)
    wUserConfigOpen.set(false)
}
export function toggleUserConfig() {
    wUserConfigOpen.update(o => !o)
    wUserModalOpen.set(false)
}
