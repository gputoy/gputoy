
import { DEFAULT_LAYOUT } from "$lib/consts/project"
import type { Layout, Panel } from "src/generated/types"
import { writable, type Writable } from "svelte/store"

export type WritableLayout = Writable<Layout> & {
    togglePanel: (panel: Panel, set?: boolean) => void
    setPanelSize: (panel: Panel, size: number) => void
    moveWorkspaceIdx: (shift: number) => void
    closeWorkspaceFile: (idx?: number) => void
}
export default function makeLayout(): WritableLayout {
    const layout = writable<Layout>(DEFAULT_LAYOUT)

    function togglePanel(panel: Panel, set?: boolean) {
        layout.update(l => {
            l[panel].show = set ?? !l[panel].show
            return l
        })
    }
    function setPanelSize(panel: Panel, size: number) {
        layout.update(l => {
            l[panel].size = size
            return l
        })
    }

    function moveWorkspaceIdx(shift: number) {
        layout.update(l => {
            const len = l.workspace.length
            if (len == 0) return l
            l.fileIndex = ((((l.fileIndex ?? 0) + shift) % len) + len) % len
            return l
        })
    }

    function closeWorkspaceFile(idx?: number) {
        layout.update(l => {
            if (!idx && !l.fileIndex) return l
            let closeIdx = idx ?? l.fileIndex!
            l.workspace.splice(closeIdx, 1)
            return l
        })
    }
    return { ...layout, togglePanel, setPanelSize, moveWorkspaceIdx, closeWorkspaceFile }
}