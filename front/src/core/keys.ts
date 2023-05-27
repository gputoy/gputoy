import { isActionEqual, pushAction } from '$core/actions'
import type { Action } from '$gen'
import { wLastInputAction, wUserKeybinds } from '$stores'
import { get } from 'svelte/store'

/**
 *  Map of keybinds to action
 */
export type Keybinds = {
	[key: string]: {
		action: Action
	}
}

/**
 * Returns list of keybinds bound for input action
 * @param find action to find binds for in keybinds
 * @returns list of bound keybinds for action
 */
export function findActionBinds(find: Action, keybinds: Keybinds): string[] {
	return Object.entries(keybinds)
		.filter(([_, action]) => isActionEqual(find, action.action))
		.map(([keybind, _]) => keybind)
}

/**
 * Returns first bind found, and null if no bind is bound for input action
 * @param find action to find bind for in keybinds
 * @returns first keybind for Action, or null if it doesn't exist
 */
export function findActionBind(
	find: Action | undefined,
	keybinds: Keybinds
): string | null {
	if (!find) return null
	const foundBinds = findActionBinds(find, keybinds)
	return foundBinds.length > 0 ? foundBinds[0] : null
}

/**
 * Transforms keyboard event to canonical keycode
 * @param ev Keyboard event
 * @returns Keybind. ex. 'C-k' is Ctrl+k, 'C-S-x' is Ctrl+Shft+x
 */
export function toKeyIdx(ev: KeyboardEvent): string {
	return (
		(ev.ctrlKey ? 'C-' : '') +
		(ev.shiftKey ? 'S-' : '') +
		(ev.altKey ? 'A-' : '') +
		ev.key.toLowerCase()
	)
}

/**
 * 'keydown' handler, attached to document on mount within (dev)/+layout.svelte
 * @param ev KeyboardEvent from listener
 * @returns void
 */
function onKeyDown(ev: KeyboardEvent) {
	if (ev.key === 'Control' || ev.key === 'Shift' || ev.key === 'Alt') return
	if (!(ev.ctrlKey || ev.shiftKey || ev.altKey)) return

	if (ev.key === 'Escape') {
		pushAction({ ty: 'hide', c: 'preferences' })
		pushAction({ ty: 'hide', c: 'debug' })
		pushAction({ ty: 'hide', c: 'user' })
	}

	const keyidx = toKeyIdx(ev)
	const action = get(wUserKeybinds)[keyidx]
	// TODO: use filtered action conditional
	if (action === undefined) return
	wLastInputAction.set({
		code: keyidx,
		action: action.action
	})
	pushAction(action.action)

	ev.preventDefault()
	ev.stopImmediatePropagation()
}

/**
 * Initializes keydown listener for document to bind user keybinds to
 * various editor actions.
 *
 * To only be used as an argument to onMount within (dev)/+layout.svelte
 * @returns event listener cleanup
 */
export function initKeys() {
	document.addEventListener('keydown', onKeyDown, { capture: true })
	return document.removeEventListener('keydown', onKeyDown)
}
