import { browser } from "$app/environment"
import type { Action } from "src/generated/types"
import { writable } from "svelte/store"
import { user } from "./auth"

const DEFAULT_KEYMAP = {
    'C-g': 'playPause'
} as {
    [key: string]: Action
}

export const actionHistory = writable<Action[]>([])
let keyMap = DEFAULT_KEYMAP

export function pushAction(action: Action) {
    console.log(action)
}

export function toKeyIdx(ev: KeyboardEvent): string {
    return (ev.ctrlKey ? 'C-' : '') + (ev.shiftKey ? 'S-' : '') + (ev.altKey ? 'A-' : '') + ev.key
}
function onKeyDown(ev: KeyboardEvent) {
    console.log(ev, ev.ctrlKey, ev.shiftKey, ev.altKey)
    if (ev.key === 'Control' || ev.key === 'Shift' || ev.key === 'Alt') return

    let keyidx = toKeyIdx(ev)
    let action = keyMap[keyidx]
    if (action === undefined) return

    pushAction(action)
    ev.preventDefault()
}

export function initKeyControls() {

    if (browser) {
        document.addEventListener("keydown", onKeyDown)
        user.subscribe(u => keyMap = u?.config?.keybinds ?? DEFAULT_KEYMAP)
        return () => document.removeEventListener("keydown", onKeyDown)
    }
}
