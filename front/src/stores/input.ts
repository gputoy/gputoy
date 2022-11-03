import type { Action } from "src/generated/types"
import { get, writable } from "svelte/store"
import { wUserConfigOpen, wUserModalOpen } from "./ui"
import { wUserKeybinds } from "./userConfig"

export const actionHistory = writable<Action[]>([])

export function pushAction(action: Action) {
    console.log(action)
}

export function toKeyIdx(ev: KeyboardEvent): string {
    return (ev.ctrlKey ? 'C-' : '') + (ev.shiftKey ? 'S-' : '') + (ev.altKey ? 'A-' : '') + ev.key
}
function onKeyDown(ev: KeyboardEvent) {
    console.log(ev, ev.ctrlKey, ev.shiftKey, ev.altKey)
    if (ev.key === 'Control' || ev.key === 'Shift' || ev.key === 'Alt') return

    if (ev.key === 'Escape') {
        wUserConfigOpen.set(false)
        wUserModalOpen.set(false)
    }

    let keyidx = toKeyIdx(ev)
    let filteredAction = get(wUserKeybinds)[keyidx]
    // TODO: use filtered action conditional
    if (filteredAction === undefined) return

    pushAction(filteredAction.action)
    ev.preventDefault()
}