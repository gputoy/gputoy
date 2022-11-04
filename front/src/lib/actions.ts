import { wLayout } from "$stores/project"
import type { Action, Panel, ShiftPaneArgs } from "src/generated/types"

const actionHistory: Action[] = []

export function pushAction(action: Action) {
    switch (action.ty) {
        case 'playPause': playPause(); break
        case 'nextDocument': shiftDoument(1); break
        case 'previousDocument': shiftDoument(-1); break
        case 'closeDocument': closeCurrentDocument(); break
        case 'rebuild': rebuildProject(); break
        case 'reset': resetProject(); break
        case 'toggleConsole': toggleConsole(); break
        case 'togglePanel': togglePanel(action.c); break
        case 'shiftPanel': shiftPanel(action.c); break
        case 'focus': focusPane(action.c); break
    }
}

export function reverseAction(action: Action) {

}

function playPause() {
}

function shiftDoument(shift: number) {
    wLayout.moveWorkspaceIdx(shift)
}

function closeCurrentDocument() {
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

function shiftPanel(c: ShiftPaneArgs) {
}

function focusPane(c: string) {
}

