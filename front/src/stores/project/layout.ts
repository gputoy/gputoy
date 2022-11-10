
import { DEFAULT_LAYOUT } from "$lib/consts/project"
import type { GeneralConfigKey } from "$lib/consts/userConfig"
import { wUserGeneralConfig } from "$stores/userConfig"
import type { Layout, Panel } from "src/generated/types"
import type { IPaneSizingEvent } from "svelte-splitpanes"
import { get } from "svelte/store"
import { makeEnhanced } from "../enhanced"

export type LayoutExtras = {
    togglePanel: (panel: Panel, set?: boolean) => void
    setPanelSize: (panel: Panel, event: IPaneSizingEvent) => void
    moveWorkspaceIdx: (shift: number) => void
    closeWorkspaceFile: (idx?: number) => void
}
export default makeEnhanced<Layout, LayoutExtras>(DEFAULT_LAYOUT, function (layout) {

    function togglePanel(panel: Panel, set?: boolean) {
        layout.update(l => {
            let tooSmallToBeOpen = l[panel].size < 12
            let expand = tooSmallToBeOpen || ((set !== undefined) ? set : !l[panel].show)
            if (tooSmallToBeOpen) l[panel].size = get(wUserGeneralConfig)[(panel + 'Size') as GeneralConfigKey]
            l[panel].show = expand
            console.log('toggling', panel, expand, l[panel])
            return l
        })
    }
    function setPanelSize(panel: Panel, event: IPaneSizingEvent) {
        layout.update(l => {
            l[panel].size = event.size
            if (event.size < event.min) l[panel].show = false
            console.log('updating: ', event)
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
            console.log(l)
            if (!idx && l.fileIndex == null) return l
            let closeIdx = idx ?? l.fileIndex!
            l.workspace.splice(closeIdx, 1)
            console.log(l.workspace)
            if (l.workspace.length > 0) l.fileIndex = Math.max(0, (l.fileIndex ?? 0) - 1)
            else l.fileIndex = null
            return l
        })
    }
    return { togglePanel, setPanelSize, moveWorkspaceIdx, closeWorkspaceFile }
}) 