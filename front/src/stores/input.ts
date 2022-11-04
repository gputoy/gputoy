import { pushAction } from "$lib/actions"
import { get } from "svelte/store"
import { wUserConfigOpen, wUserModalOpen } from "./ui"
import { wUserKeybinds } from "./userConfig"

export function toKeyIdx(ev: KeyboardEvent): string {
    return (ev.ctrlKey ? 'C-' : '') + (ev.shiftKey ? 'S-' : '') + (ev.altKey ? 'A-' : '') + ev.key
}
function onKeyDown(ev: KeyboardEvent) {
    if (ev.key === 'Control' || ev.key === 'Shift' || ev.key === 'Alt') return

    if (ev.key === 'Escape') {
        wUserConfigOpen.set(false)
        wUserModalOpen.set(false)
    }

    let keyidx = toKeyIdx(ev)
    console.log('Got key', keyidx)
    let filteredAction = get(wUserKeybinds)[keyidx]
    // TODO: use filtered action conditional
    if (filteredAction === undefined) return
    console.log('Got action: ', filteredAction.action)
    pushAction(filteredAction.action)
    ev.preventDefault()
    ev.stopImmediatePropagation()
}
export function initKeys() {
    document.addEventListener('keydown', onKeyDown, { capture: true })
    return document.removeEventListener('keydown', onKeyDown)
}