import type { Layout, Panel } from '$common'
import { DEFAULT_DIR_NODE_STATE, type GeneralConfigKey } from '$core/consts'
import { wUserGeneralPrefs } from '$stores'
import type { IPaneSizingEvent } from 'svelte-splitpanes'
import { get, type Writable } from 'svelte/store'

export type LayoutExtras = {
	getOpenFileId: () => string | undefined
	setPanelSize: (panel: Panel, event: IPaneSizingEvent) => void
	moveWorkspaceIdx: (shift: number) => void
	closeWorkspaceFile: (idx?: number) => void
	openDocument: (fileid: string) => void
	togglePanel: (panel: Panel, set?: boolean) => void
	toggleDirOpen: (absoluteDir: string, set?: boolean) => void
	toggleAccordian: (accordian: string, set?: boolean) => void
}
export function initLayoutMethods(layout: Writable<Layout>): LayoutExtras {
	function getOpenFileId(): string | undefined {
		const _layout = get(layout)
		return _layout.fileIndex != null ? _layout.workspace[_layout.fileIndex] : undefined
	}

	function setPanelSize(panel: Panel, event: IPaneSizingEvent) {
		layout.update((l) => {
			l[panel].size = event.size
			if (event.size < event.min) l[panel].show = false
			return l
		})
	}

	function moveWorkspaceIdx(shift: number) {
		layout.update((l) => {
			const len = l.workspace.length
			if (len == 0) return l
			l.fileIndex = ((((l.fileIndex ?? 0) + shift) % len) + len) % len
			return l
		})
	}

	function closeWorkspaceFile(idx?: number) {
		layout.update((l) => {
			if (!idx && l.fileIndex == null) return l
			const closeIdx = idx ?? l.fileIndex!
			l.workspace.splice(closeIdx, 1)
			if (l.workspace.length > 0)
				l.fileIndex = Math.max(0, (l.fileIndex ?? 0) - 1)
			else l.fileIndex = null
			return l
		})
	}

	function openDocument(fileid: string) {
		layout.update((l) => {
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
		layout.update((l) => {
			const tooSmallToBeOpen = l[panel].size < 12
			const expand =
				tooSmallToBeOpen || (set !== undefined ? set : !l[panel].show)
			if (tooSmallToBeOpen)
				l[panel].size = get(wUserGeneralPrefs)[
					(panel + 'Size') as GeneralConfigKey
				] as number
			l[panel].show = expand
			return l
		})
	}

	function toggleDirOpen(absoluteDir: string, set?: boolean) {
		layout.update((l) => {
			if (!l.fileTreeState[absoluteDir]) {
				l.fileTreeState[absoluteDir] = DEFAULT_DIR_NODE_STATE
			}
			const open = set === undefined ? !l.fileTreeState[absoluteDir].open : set
			l.fileTreeState[absoluteDir] = { ...l.fileTreeState[absoluteDir], open }
			return l
		})
	}

	function toggleAccordian(accordian: string, set?: boolean) {
		layout.update((l) => {
			l.accordianOpen[accordian] =
				set === undefined ? !l.accordianOpen[accordian] : set
			return l
		})
	}

	return {
		getOpenFileId,
		setPanelSize,
		moveWorkspaceIdx,
		closeWorkspaceFile,
		openDocument,
		togglePanel,
		toggleDirOpen,
		toggleAccordian
	}
}
