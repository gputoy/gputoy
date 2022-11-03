import { DEFAULT_PANEL_SIZES, DEFAULT_PANEL_TOGGLE } from '$lib/consts/ui'
import { writable } from 'svelte/store'

export const wUserModalOpen = writable(false)
export const wUserConfigOpen = writable(false)

/**
 * Percentage of splitpane taken up by panel
 */
export type PanelSizes = {
    projectPanel: number,
    resourcePanel: number,
    editorPanel: number,
}
export const wPanelSizes = writable<PanelSizes>(DEFAULT_PANEL_SIZES)

export type PanelToggle = {
    projectPanel: boolean,
    resourcePanel: boolean,
    editorPanel: boolean,
}
export const wPanelToggle = writable<PanelToggle>(DEFAULT_PANEL_TOGGLE)

export function toggleUserModal() {
    wUserModalOpen.update(o => !o)
    wUserConfigOpen.set(false)
}
export function toggleUserConfig() {
    wUserConfigOpen.update(o => !o)
    wUserModalOpen.set(false)
}
