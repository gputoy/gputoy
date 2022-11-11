import { clearProject, wLayout } from "$stores/project"
import { wDebugPanel } from "$stores/ui"
import { toast } from "@zerodevx/svelte-toast"
import { isEqual } from "lodash"
import type { Action, FilteredAction, Panel, ShiftPaneArgs } from "src/generated/types"

const actionHistory: Action[] = []

/**
 * Determines if two Actions are equal by value 
 * @param a 
 * @param b 
 * @returns true/false if actions are equal by value 
 */
export function isActionEqual(a: Action, b: Action): boolean {
    if (a.ty !== b.ty) return false
    // Both have action arguments within their type variant
    if ('c' in a && 'c' in b)
        return isEqual(a.c, b.c)
    return true
}

/**
 * Evaluates filter from filteredAction
 * TODO: implement specific filters
 * @param filter 
 * @returns 
 */
export function actionPermitted(fAction: FilteredAction) {
    return true
}

export const FILTER_CONDITION_LIST = [
    'userOwnsProject',
    'userLoggedIn',

    'filesDirty',
    'fileDirty',
    'fileOpen',

    'editorPanelFocused',
    'resourcePanelFocused',
    'projectPanelFocused',
    'viewportPanelFocused',
]
export type SingleFilter = typeof FILTER_CONDITION_LIST[number]

/**
 * Pushes Action to be executed
 * @param action Action to be executed
 */
export function pushAction(action: Action) {
    switch (action.ty) {
        case 'playPause': playPause(); break
        case 'openDocument': openDocument(action.c); break
        case 'nextDocument': shiftDoument(1); break
        case 'previousDocument': shiftDoument(-1); break
        case 'rebuild': rebuildProject(); break
        case 'reset': resetProject(); break
        case 'toggleConsole': toggleConsole(); break
        case 'togglePanel': togglePanel(action.c); break
        case 'shiftPanel': shiftPanel(action.c); break
        case 'focus': focusPane(action.c); break
        case 'toggleDebugPanel': toggleDebugPanel(); break
        case 'closeFile': closeCurrentFile(); break
        case 'closeProject': closeProject(); break

        /** @ts-ignore */
        // There may be a case in the future where a new variant is added
        // and the implementation was forgotten
        default: toast.push('Action unrecognized or not yet implemented: ' + action.ty)
    }
}

/**
 *  TODO: create action reversal system 
 * @param action 
 */
export function reverseAction(action: Action) {

}


/// ------------------- Action execution ----------------------

function playPause() {
}

function openDocument(fileid: string) {
    wLayout.openDocument(fileid)
}
function shiftDoument(shift: number) {
    wLayout.moveWorkspaceIdx(shift)
}

function closeCurrentFile() {
    console.log('in close file')
    wLayout.closeWorkspaceFile()
}

function rebuildProject() {
}

function resetProject() {
}

function toggleConsole() {
}

function togglePanel(panel: Panel) {
    wLayout.togglePanel(panel)
}

function toggleDebugPanel() {
    wDebugPanel.update(show => !show)
}

function shiftPanel(c: ShiftPaneArgs) {
}

function focusPane(c: string) {
}

function closeProject() {
    clearProject()
}