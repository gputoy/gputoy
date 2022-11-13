
import { DEFAULT_DIR_NODE_STATE, DEFAULT_LAYOUT } from "$lib/consts/project"
import type { GeneralConfigKey } from "$lib/consts/userConfig"
import { wUserGeneralConfig } from "$stores/userConfig"
import type { Layout, Panel } from "src/generated/types"
import type { IPaneSizingEvent } from "svelte-splitpanes"
import { get } from "svelte/store"
import { makeEnhanced } from "../enhanced"

export type LayoutExtras = {
    setPanelSize: (panel: Panel, event: IPaneSizingEvent) => void
    moveWorkspaceIdx: (shift: number) => void
    closeWorkspaceFile: (idx?: number) => void
    openDocument: (fileid: string) => void
    togglePanel: (panel: Panel, set?: boolean) => void
    toggleDirOpen: (absoluteDir: string, set?: boolean) => void
    toggleAccordian: (accordian: string, set?: boolean) => void
}
export default makeEnhanced<Layout, LayoutExtras>(DEFAULT_LAYOUT, function (layout) {

    function setPanelSize(panel: Panel, event: IPaneSizingEvent) {
        layout.update(l => {
            l[panel].size = event.size
            if (event.size < event.min) l[panel].show = false
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
            if (!idx && l.fileIndex == null) return l
            let closeIdx = idx ?? l.fileIndex!
            l.workspace.splice(closeIdx, 1)
            if (l.workspace.length > 0) l.fileIndex = Math.max(0, (l.fileIndex ?? 0) - 1)
            else l.fileIndex = null
            return l
        })
    }

    function openDocument(fileid: string) {
        layout.update(l => {
            let maybeIndex = l.workspace.indexOf(fileid)
            // add file to workspace
            if (maybeIndex < 0) {
                maybeIndex = l.workspace.push(fileid) - 1
            }
            l.fileIndex = maybeIndex
            return l
        })
    }

    function togglePanel(panel: Panel, set?: boolean) {
        layout.update(l => {
            let tooSmallToBeOpen = l[panel].size < 12
            let expand = tooSmallToBeOpen || ((set !== undefined) ? set : !l[panel].show)
            if (tooSmallToBeOpen) l[panel].size = get(wUserGeneralConfig)[(panel + 'Size') as GeneralConfigKey]
            l[panel].show = expand
            return l
        })
    }

    function toggleDirOpen(absoluteDir: string, set?: boolean) {
        layout.update(l => {
            if (!l.fileTreeState[absoluteDir]) {
                l.fileTreeState[absoluteDir] = DEFAULT_DIR_NODE_STATE
            }
            const open = (set === undefined) ? !l.fileTreeState[absoluteDir].open : set
            l.fileTreeState[absoluteDir] = { ...l.fileTreeState[absoluteDir], open }
            return l
        })
    }

    function toggleAccordian(accordian: string, set?: boolean) {
        layout.update(l => {
            l.accordianOpen[accordian] = (set === undefined) ? !l.accordianOpen[accordian] : set
            return l
        })
    }

    return { setPanelSize, moveWorkspaceIdx, closeWorkspaceFile, openDocument, togglePanel, toggleDirOpen, toggleAccordian }
}) 