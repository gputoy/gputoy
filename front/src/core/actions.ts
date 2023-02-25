import type { Action, FilteredAction, Pane } from '$common'
import { clearProject } from '$core/project'
import { getAllModels, getModel } from '$monaco'
import { wWorkerInternal } from '$monaco/wgsl/wgslMode'
import {
	wConfig,
	wConsole,
	wConsoleOpen,
	wDebugPanel,
	wFileDirty,
	wFiles,
	wModelDirty,
	wRunState,
	wWorkerData
} from '$stores'
import { toast } from '@zerodevx/svelte-toast'
import isEqual from 'lodash/isEqual'
import { fileWithNewPath } from './files'
import {
	closeWorkspaceFile,
	getOpenFileId,
	moveWorkspaceIdx,
	openDocument as layoutOpenDocument,
	toggleAllPanels as layoutToggleAllPanels,
	togglePanel as layoutTogglePanel
} from './layout'

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
	if ('c' in a && 'c' in b) return isEqual(a.c, b.c)
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
	'viewportPanelFocused'
]
export type SingleFilter = (typeof FILTER_CONDITION_LIST)[number]

/**
 * Pushes Action to be executed
 * @param action Action to be executed
 */
export function pushAction(action: Action) {
	switch (action.ty) {
		case 'playPause':
			playPause()
			break
		case 'openDocument':
			openDocument(action.c)
			break
		case 'nextDocument':
			shiftDoument(1)
			break
		case 'previousDocument':
			shiftDoument(-1)
			break
		case 'rebuild':
			rebuildProject()
			break
		case 'reset':
			resetProject()
			break
		case 'toggleConsole':
			toggleConsole()
			break
		case 'togglePanel':
			togglePanel(action.c)
			break
		case 'toggleAllPanels':
			toggleAllPanels()
			break
		case 'focus':
			focusPane(action.c)
			break
		case 'toggleDebugPanel':
			toggleDebugPanel()
			break
		case 'closeFile':
			closeCurrentFile()
			break
		case 'closeProject':
			closeProject()
			break
		case 'setRunner':
			setRunner(action.c)
			break
		case 'saveCurrentFile':
			saveCurrentFile()
			break
		case 'saveAllFiles':
			saveAllFiles()
			break
		case 'move':
			moveFile(action.c[0], action.c[1])
			break

		/** @ts-ignore */
		// There may be a case in the future where a new variant is added
		// and the implementation was forgotten
		default:
			toast.push('Action unrecognized or not yet implemented: ' + action.ty)
	}
}

/**
 *  TODO: create action reversal system
 * @param action
 */
export function reverseAction(action: Action) {}

/// ------------------- Action execution ----------------------

async function playPause() {
	// if (get(wPrebuildDirty)) {
	//     context.prebuild()
	// }

	// if (get(wBuildDirty)) {
	//     context.build()
	// }
	wWorkerInternal.tryBuild()
	wWorkerData.set(await wWorkerInternal.poll())
	wRunState.playPause()
}

function openDocument(fileid: string) {
	layoutOpenDocument(fileid)
}
function shiftDoument(shift: number) {
	moveWorkspaceIdx(shift)
}

function closeCurrentFile() {
	closeWorkspaceFile()
}

function rebuildProject() {}

function resetProject() {}

function toggleConsole() {
	wConsoleOpen.update((o) => !o)
}

function togglePanel(panel: Pane) {
	layoutTogglePanel(panel)
}

function toggleAllPanels() {
	layoutToggleAllPanels()
}

function toggleDebugPanel() {
	wDebugPanel.update((show) => !show)
}

function focusPane(c: string) {}

function closeProject() {
	clearProject()
}

function setRunner(fileid: string) {
	if (wFiles.getFile(fileid))
		wConfig.update((config) => {
			config.runner = fileid
			return config
		})
	else wConsole.error('Runner not found: ' + fileid)
}

async function saveCurrentFile() {
	const currentFile = getOpenFileId()
	if (!currentFile) return
	await wWorkerInternal.applyUpdateToFile(currentFile)
	wModelDirty.remove(currentFile)

	const model = getModel(currentFile)
	if (!model) return
	wFiles.writeFile(currentFile, model.getValue())
	wFileDirty.remove(currentFile)
}

async function saveAllFiles() {
	await wWorkerInternal.applyUpdateToFile()
	wModelDirty.clear()

	getAllModels().forEach((model) => {
		const path = model.uri.path
		wFiles.writeFile(path, model.getValue())
	})
	wFileDirty.clear()
}

function moveFile(src: string, dest: string) {
	console.log('in moveFile', src, dest)

	let didUpdate = false
	wFiles.update(({ map }) => {
		let curr = map[src]
		if (curr && map[dest] == undefined) {
			const newFile = fileWithNewPath(curr, dest)
			if (!newFile) return { map }
			map[dest] = newFile
			delete map[src]
			didUpdate = true
		}
		return { map }
	})
	if (didUpdate) {
	}
}
