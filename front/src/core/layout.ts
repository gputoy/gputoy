import { browser } from '$app/environment'
import { DEFAULT_DIR_NODE_STATE, DEFAULT_LAYOUT as DEFAULT } from '$core/consts'
import type { Layout, Region } from '$gen'
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
const wFileTreeState = writable<(typeof L)['fileTreeState']>(
	DEFAULT['fileTreeState']
)
const wPaneSize = writable<(typeof L)['paneSize']>(DEFAULT['paneSize'])
const wPaneToggle = writable<(typeof L)['paneToggled']>(DEFAULT['paneToggled'])
const wTabs = writable<(typeof L)['tabs']>(DEFAULT['tabs'])
const wTabIndex = writable<(typeof L)['tabIndex']>(DEFAULT['tabIndex'])

/**
 * Public readable atomic stores of the Layout struct
 */
export const rAccordianOpen = sealWritable(
	wAccordianOpen,
	'layout.accordian-open'
)
export const rFileTreeState = sealWritable(wFileTreeState, 'layout.file-tree')
export const rPaneSize = sealWritable(wPaneSize, 'layout.pane-size')
export const rPaneToggle = sealWritable(wPaneToggle, 'layout.pane-toggle')
export const rTabs = sealWritable(wTabs, 'layout.tabs')
export const rTabIndex = sealWritable(wTabIndex, 'layout.tab-index')

// whether the window size is currently being updated
export const wUpdatingWindowSize = writable(false)
// window size - top navbar height
export const wWindowWidth = writable<number | undefined>()
export const wWindowHeight = writable<number | undefined>()

// other ui regions that need state to track if they're open
// these are not included in the Layout struct since there is no point
// to serializing them (i.e. user prefs shouldn't stay open across sessions)
const wUserModalOpen = writable(false)
const wUserPrefsOpen = writable(false)
const wTerminalOpen = writable(false)
const wDebugOpen = writable(false)

export const rUserModalOpen = sealWritable(
	wUserModalOpen,
	'layout.user-modal-open'
)
export const rUserPrefsOpen = sealWritable(
	wUserPrefsOpen,
	'layout.user-prefs-open'
)
export const rTerminalOpen = sealWritable(wTerminalOpen, 'layout.terminal-open')
export const rDebugOpen = sealWritable(wDebugOpen, 'layout.debug-open')

// Raw pane sizes, tracked by on:resize via the svelte component
// and the forceUpdateRawPaneSize function below
var _rawPaneEvents: any = {
	projectPane: undefined,
	editorPane: undefined,
	resourcePane: undefined
}

function forceUpdateRawPaneSize(region: Region, ev: Partial<IPaneSizingEvent>) {
	_rawPaneEvents[region] = { ..._rawPaneEvents[region], ...ev }
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
	[rAccordianOpen, rFileTreeState, rPaneSize, rPaneToggle, rTabIndex, rTabs],
	([accordianOpen, fileTreeState, paneSize, paneToggled, tabIndex, tabs]) => {
		return {
			accordianOpen,
			fileTreeState,
			paneSize,
			paneToggled,
			tabIndex,
			tabs
		}
	}
)

export const dActiveFile = derived(
	[rTabIndex, rTabs],
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

		const { projectPanePx, editorPanePercentage, controlPanePercentage } =
			get(wPaneSize)

		let projectPanePct = (projectPanePx / (width ?? 1000)) * 100
		if (toggle['project-pane'])
			forceUpdateRawPaneSize('project-pane', { size: projectPanePct })
		projectPanePct = toggle['project-pane'] ? projectPanePct : 0

		let editorPanePct = toggle['editor-pane'] ? editorPanePercentage : 0
		if (toggle['editor-pane'])
			forceUpdateRawPaneSize('editor-pane', { size: editorPanePct })
		let centerPanePct = 100 - projectPanePct - editorPanePct

		let controlBarMinSize: number =
			(CONTROL_BAR_HEIGHT / (height ?? 1000)) * 100
		let controlPanePct = toggle['control-pane']
			? controlPanePercentage
			: controlBarMinSize
		if (toggle['control-pane'])
			forceUpdateRawPaneSize('control-pane', { size: controlPanePct })
		let viewportPanePct = 100 - controlPanePct

		const ret = {
			projectPanePx,
			projectPanePct,
			editorPanePct,
			centerPanePct,
			viewportPanePct,
			controlPanePct,
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
	wTabs.set(layout.tabs)
	wTabIndex.set(layout.tabIndex)
}

/**
 *  Layout methods
 */

export function getOpenFileId(): string | undefined {
	const fileIndex = get(rTabIndex)
	const workspace = get(rTabs)
	return fileIndex != null ? workspace[fileIndex] : undefined
}

export function setFileIndex(index: number) {
	wTabIndex.set(index)
}

export function shiftTab(shift: number) {
	wTabIndex.update((fileIndex) => {
		const len = get(rTabs).length
		if (len == 0) return fileIndex
		return ((((fileIndex ?? 0) + shift) % len) + len) % len
	})
}

export function closeTab(idx?: number) {
	let fileIndex = get(rTabIndex)
	let workspace = get(rTabs)
	if (!idx && fileIndex == null) return
	const closeIdx = idx ?? fileIndex!
	workspace.splice(closeIdx, 1)
	if (workspace.length > 0) fileIndex = Math.max(0, (fileIndex ?? 0) - 1)
	else fileIndex = null
	wTabIndex.set(fileIndex)
	wTabs.set(workspace)
}

export function openTab(fileid: string) {
	let maybeIndex
	wTabs.update((tabs) => {
		maybeIndex = tabs.indexOf(fileid)
		// add file to workspace
		if (maybeIndex < 0) {
			maybeIndex = tabs.push(fileid) - 1
		}
		return tabs
	})
	wTabIndex.set(maybeIndex)
}

const APPLY_PANE_SIZE_WAIT_MS = 300
const PIXEL_MIN = 100
const PERCENTAGE_MIN = 15
const setSizes = debounce(_setSizes, APPLY_PANE_SIZE_WAIT_MS)
function _setSizes(p?: number, e?: number, r?: number) {
	wPaneSize.update((size) => ({
		projectPanePx: p ?? size.projectPanePx,
		controlPanePercentage: r ?? size.controlPanePercentage,
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
export function toggleRegionVisibility(region: Region) {
	const BIAS_PERCENT = 5
	if (
		region == 'project-pane' ||
		region == 'control-pane' ||
		region == 'editor-pane'
	) {
		const toggle = () =>
			wPaneToggle.update((toggle) => {
				toggle[region] = !toggle[region]
				return toggle
			})

		let lastEvent = _rawPaneEvents[region] as IPaneSizingEvent
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
	} else {
		const toggle = (val: boolean) => {
			return !val
		}
		;({
			preferences: () => {
				wUserPrefsOpen.update(toggle)
			},
			terminal: () => {
				wTerminalOpen.update(toggle)
			},
			debug: () => {
				wDebugOpen.update(toggle)
			},
			user: () => {
				wUserModalOpen.update(toggle)
			}
		})[region]()
	}
}
export function setRegionVisibility(region: Region, visibility: boolean) {
	if (
		region == 'project-pane' ||
		region == 'control-pane' ||
		region == 'editor-pane'
	) {
		wPaneToggle.update((toggle) => {
			toggle[region] = visibility
			return toggle
		})
	} else {
		;({
			preferences: () => {
				wUserPrefsOpen.set(visibility)
			},
			terminal: () => {
				wTerminalOpen.set(visibility)
			},
			debug: () => {
				wDebugOpen.set(visibility)
			},
			user: () => {
				wUserModalOpen.set(visibility)
			}
		})[region]()
	}
}

export function toggleAllPanes() {
	let {
		'editor-pane': e,
		'project-pane': p,
		'control-pane': c
	} = get(wPaneToggle)
	if (e || p || c)
		wPaneToggle.set({
			'editor-pane': false,
			'project-pane': false,
			'control-pane': false
		})
	else
		wPaneToggle.set({
			'editor-pane': true,
			'project-pane': true,
			'control-pane': true
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

export function replaceIdInTabs(src: string, dest: string) {
	wTabs.update((tabs) => {
		tabs = tabs.map((fileid) => (fileid == src ? dest : fileid))
		return tabs
	})
}

export function deleteIdInTabs(fileid: string) {
	wTabs.update((tabs) => {
		tabs = tabs.filter((id) => id != fileid)
		return tabs
	})
}
