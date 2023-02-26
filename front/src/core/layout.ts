import { browser } from '$app/environment'
import type { Layout, Pane } from '$common'
import { DEFAULT_DIR_NODE_STATE, DEFAULT_LAYOUT as DEFAULT } from '$core/consts'
import { sealWritable } from '$stores'
import debounce from 'lodash/debounce'
import type { IPaneSizingEvent } from 'svelte-splitpanes'
import { derived, get, writable, type Readable } from 'svelte/store'

declare var L: Layout

/**
 * Atomic stores of the Layout struct
 */
const wAccordianOpen = writable<(typeof L)['accordianOpen']>(
	DEFAULT['accordianOpen']
)
const wFileIndex = writable<(typeof L)['fileIndex']>(DEFAULT['fileIndex'])
const wFileTreeState = writable<(typeof L)['fileTreeState']>(
	DEFAULT['fileTreeState']
)
const wPaneSize = writable<(typeof L)['paneSize']>(DEFAULT['paneSize'])
const wPaneToggle = writable<(typeof L)['paneToggled']>(DEFAULT['paneToggled'])
const wWorkspace = writable<(typeof L)['workspace']>(DEFAULT['workspace'])

/**
 * Public readable atomic stores of the Layout struct
 */
export const rAccordianOpen = sealWritable(wAccordianOpen)
export const rFileIndex = sealWritable(wFileIndex)
export const rFileTreeState = sealWritable(wFileTreeState)
export const rPaneSize = sealWritable(wPaneSize)
export const rPaneToggle = sealWritable(wPaneToggle)
export const rWorkspace = sealWritable(wWorkspace)

// whether the window size is currently being updated
export const wUpdatingWindowSize = writable(false)
// window size - top navbar height
export const wWindowWidth = writable<number | undefined>()
export const wWindowHeight = writable<number | undefined>()

// Raw pane sizes, tracked by on:resize via the svelte component
// and the forceUpdateRawPaneSize function below
var _rawPaneEvents: any = {
	projectPane: undefined,
	editorPane: undefined,
	resourcePane: undefined
}

function forceUpdateRawPaneSize(pane: Pane, ev: Partial<IPaneSizingEvent>) {
	_rawPaneEvents[pane] = { ..._rawPaneEvents[pane], ...ev }
}

// Due to css transition, some panes were floating around when the
// window width was changed. This disables the transition and re-enables it
// shortly after if the resize has ended.
var timeout: number | undefined
wWindowWidth.subscribe((_) => {
	if (!browser) return
	if (timeout) window.clearTimeout(timeout)
	wUpdatingWindowSize.set(true)
	timeout = window.setTimeout(() => {
		wUpdatingWindowSize.set(false)
	}, 10)
})

// Version writable for triggering the dPaneSizes derive
// without changing the toggles
const wVersionBump = writable(0)

export const dLayout: Readable<Layout> = derived(
	[
		rAccordianOpen,
		rFileIndex,
		rFileTreeState,
		rPaneSize,
		rPaneToggle,
		rWorkspace
	],
	([
		accordianOpen,
		fileIndex,
		fileTreeState,
		paneSize,
		paneToggled,
		workspace
	]) => {
		return {
			accordianOpen,
			fileIndex,
			fileTreeState,
			paneSize,
			paneToggled,
			workspace
		}
	}
)

export const dActiveFile = derived(
	[rFileIndex, rWorkspace],
	([fileIndex, workspace]): string | null => {
		if (fileIndex == null) return null
		return workspace[fileIndex]
	}
)

export const dPaneSizes = derived(
	[wPaneToggle, wWindowWidth, wWindowHeight, wVersionBump],
	([toggle, width, height, version]) => {
		// none of the stores have been initalized yet
		if (!toggle) return undefined

		const CONTROL_BAR_HEIGHT: number = 32

		const { projectPanePx, editorPanePercentage, resourcePanePercentage } =
			get(wPaneSize)

		let projectPanePct = (projectPanePx / (width ?? 1000)) * 100
		if (toggle.projectPane)
			forceUpdateRawPaneSize('projectPane', { size: projectPanePct })
		projectPanePct = toggle.projectPane ? projectPanePct : 0

		let editorPanePct = toggle.editorPane ? editorPanePercentage : 0
		if (toggle.editorPane)
			forceUpdateRawPaneSize('editorPane', { size: editorPanePct })
		let centerPanePct = 100 - projectPanePct - editorPanePct

		let controlBarMinSize = (CONTROL_BAR_HEIGHT / (height ?? 1000)) * 100
		let resourcePanePct = toggle.resourcePane
			? resourcePanePercentage
			: controlBarMinSize
		if (toggle.resourcePane)
			forceUpdateRawPaneSize('resourcePane', { size: resourcePanePct })
		let viewportPanePct = 100 - resourcePanePct

		const ret = {
			projectPanePx,
			projectPanePct,
			editorPanePct,
			centerPanePct,
			viewportPanePct,
			resourcePanePct,
			controlBarMinSize,
			version
		}

		return ret
	}
)

export function getLayout(): Layout {
	return get(dLayout)
}
export function loadLayout(layout: Layout) {
	wAccordianOpen.set(layout.accordianOpen)
	wFileTreeState.set(layout.fileTreeState)
	wPaneSize.set(layout.paneSize)
	wPaneToggle.set(layout.paneToggled)
	wWorkspace.set(layout.workspace)
	wFileIndex.set(layout.fileIndex)
}

/**
 *  Layout methods
 */

export function getOpenFileId(): string | undefined {
	const fileIndex = get(rFileIndex)
	const workspace = get(rWorkspace)
	return fileIndex != null ? workspace[fileIndex] : undefined
}

export function setFileIndex(index: number) {
	wFileIndex.set(index)
}

export function moveWorkspaceIdx(shift: number) {
	wFileIndex.update((fileIndex) => {
		const len = get(rWorkspace).length
		if (len == 0) return fileIndex
		return ((((fileIndex ?? 0) + shift) % len) + len) % len
	})
}

export function closeWorkspaceFile(idx?: number) {
	let fileIndex = get(rFileIndex)
	let workspace = get(rWorkspace)
	if (!idx && fileIndex == null) return
	const closeIdx = idx ?? fileIndex!
	workspace.splice(closeIdx, 1)
	if (workspace.length > 0) fileIndex = Math.max(0, (fileIndex ?? 0) - 1)
	else fileIndex = null
	wFileIndex.set(fileIndex)
	wWorkspace.set(workspace)
}

export function openDocument(fileid: string) {
	let maybeIndex
	wWorkspace.update((workspace) => {
		maybeIndex = workspace.indexOf(fileid)
		// add file to workspace
		if (maybeIndex < 0) {
			maybeIndex = workspace.push(fileid) - 1
		}
		return workspace
	})
	wFileIndex.set(maybeIndex)
}

const APPLY_PANE_SIZE_WAIT_MS = 300
const PIXEL_MIN = 100
const PERCENTAGE_MIN = 15
const setSizes = debounce(_setSizes, APPLY_PANE_SIZE_WAIT_MS)
function _setSizes(p?: number, e?: number, r?: number) {
	wPaneSize.update((size) => ({
		projectPanePx: p ?? size.projectPanePx,
		resourcePanePercentage: r ?? size.resourcePanePercentage,
		editorPanePercentage: e ?? size.editorPanePercentage
	}))
}
export function applySplitpaneEvent(event: CustomEvent<IPaneSizingEvent[]>) {
	// Horizontal resize
	if (event.detail.length == 3) {
		const width = get(wWindowWidth)
		const [projectPane, _, editorPane] = event.detail
		_rawPaneEvents.projectPane = projectPane
		_rawPaneEvents.editorPane = editorPane
		let projectPanePx: number | undefined =
			(projectPane.size * (width ?? 1000)) / 100
		projectPanePx = projectPanePx > PIXEL_MIN ? projectPanePx : undefined
		let editorPanePct: number | undefined =
			editorPane.size > PERCENTAGE_MIN ? editorPane.size : undefined
		setSizes(projectPanePx, editorPanePct)
	}
	// Veritcal resize
	else if (event.detail.length == 2) {
		const [_, resourcePane] = event.detail
		_rawPaneEvents.resourcePane = resourcePane
		let resourcePanePct =
			resourcePane.size > PERCENTAGE_MIN ? resourcePane.size : undefined
		setSizes(undefined, undefined, resourcePanePct)
	}
}
const PROJECT_PANE_PIXEL_THRESHOLD = 100
const BIAS_PERCENT = 5
export function togglePanel(pane: Pane) {
	const toggle = () =>
		wPaneToggle.update((toggle) => {
			toggle[pane] = !toggle[pane]
			return toggle
		})

	let lastEvent = _rawPaneEvents[pane] as IPaneSizingEvent
	if (!lastEvent) return toggle()
	let tooSmallToOpen =
		Math.max(lastEvent.min, lastEvent.snap) + BIAS_PERCENT > lastEvent.size

	if (tooSmallToOpen) {
		wVersionBump.update((v) => {
			return v + 1
		})
	} else {
		toggle()
	}
}

export function toggleAllPanels() {
	let { editorPane, projectPane, resourcePane } = get(wPaneToggle)
	if (editorPane || projectPane || resourcePane)
		wPaneToggle.set({
			editorPane: false,
			projectPane: false,
			resourcePane: false
		})
	else
		wPaneToggle.set({
			editorPane: true,
			projectPane: true,
			resourcePane: true
		})
}

export function toggleDirOpen(absoluteDir: string, set?: boolean) {
	wFileTreeState.update((tree) => {
		if (!tree[absoluteDir]) {
			tree[absoluteDir] = DEFAULT_DIR_NODE_STATE
		}
		const open = set === undefined ? !tree[absoluteDir].open : set
		tree[absoluteDir] = { ...tree[absoluteDir], open }
		return tree
	})
}

export function moveFileTreeState(src: string, dest: string) {
	wFileTreeState.update((tree) => {
		if (!tree[src]) {
			tree[dest] = DEFAULT_DIR_NODE_STATE
			return tree
		}
		for (const fileid of Object.keys(tree)) {
			if (fileid.startsWith(src)) {
				let newId = fileid.replace(src, dest)
				tree[newId] = tree[fileid]
				delete tree[fileid]
			}
		}
		return tree
	})
}

export function toggleAccordian(accordian: string, set?: boolean) {
	wAccordianOpen.update((accordianOpen) => {
		accordianOpen[accordian] =
			set === undefined ? !accordianOpen[accordian] : set
		return accordianOpen
	})
}

export function replaceIdInWorkspace(src: string, dest: string) {
	wWorkspace.update((workspace) => {
		workspace = workspace.map((fileid) => (fileid == src ? dest : fileid))
		return workspace
	})
}

export function deleteIdInWorkspace(fileid: string) {
	wWorkspace.update((workspace) => {
		workspace = workspace.filter((id) => id != fileid)
		return workspace
	})
}
