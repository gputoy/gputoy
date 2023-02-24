import type { File, Layout, SupportedExtension, UserEditorPrefs } from '$common'
import type * as monaco from 'monaco-editor'
import editorWorker from 'monaco-editor/esm/vs/editor/editor.worker?worker'
import jsonWorker from 'monaco-editor/esm/vs/language/json/json.worker?worker'
import gputWorker from './wgsl/worker?worker'

import type { Theme } from '$core/util'
import { wFiles } from '$stores'
import { writable } from 'svelte/store'
import dark from './dark'
import setJSONSchema from './json'
import light from './light'
import Statusbar from './statusbar'

export const cursorPosition =
	writable<monaco.editor.ICursorPositionChangedEvent | null>(null)
export const currentExtension = writable<SupportedExtension | null>(null)

let Monaco: typeof import('monaco-editor')
declare let self: { MonacoEnvironment: any }

/** @ts-ignore*/
let MonacoVim: any
let _editorInstance: monaco.editor.IStandaloneCodeEditor | undefined = undefined
const _cachedPositions: { [key: string]: monaco.Position | undefined } = {}
let _vimMode: any | undefined = undefined

/** @ts-ignore*/
let _divEl: HTMLDivElement
/** @ts-ignore*/
let _statusEl: HTMLDivElement

export var noInstance = _editorInstance === undefined

export type EditorInit = {
	theme: Theme
	layout: Layout
	prefs: UserEditorPrefs
}
// Initializes the monaco editor instance
export async function initEditor(divEl: any, statusEl: any, init: EditorInit) {
	_divEl = divEl
	_statusEl = statusEl

	self.MonacoEnvironment = {
		getWorker(_workerid: string, label: string) {
			if (label === 'json') {
				return new jsonWorker()
			} else if ((label = 'wgsl')) {
				return new gputWorker()
			}
			return new editorWorker()
		}
	}
	Monaco = await import('monaco-editor')
	await import('./wgsl')
	/** @ts-ignore */
	MonacoVim = await import('monaco-vim')
	Monaco.editor.defineTheme('dark', dark())
	Monaco.editor.defineTheme('light', light)
	setJSONSchema(Monaco)

	_editorInstance = Monaco.editor.create(divEl, {
		automaticLayout: true,
		minimap: {
			enabled: false
		},
		guides: {
			highlightActiveBracketPair: true,
			bracketPairs: true,
			indentation: true,
			highlightActiveIndentation: true,
			bracketPairsHorizontal: true
		},
		padding: {
			bottom: 20,
			top: 20
		}
	})

	// manually set theme and file on init
	Monaco.editor.setTheme(init.theme)
	changeFileFromLayout(init.layout)
	updateEditorConfig(init.prefs)
	_editorInstance.onDidChangeCursorPosition(cursorPosition.set)

	// On destroy, dispose editor
	return () => {
		_editorInstance?.dispose()
	}
}

// Finds which file to display based on layout
export function changeFileFromLayout(layout: Layout) {
	if (!Monaco) return
	if (layout.fileIndex == null) return
	const fileid = layout.workspace[layout.fileIndex]
	if (!fileid) return
	const file = wFiles.getFile(fileid)
	currentExtension.set(file?.extension ?? null)
	if (file) changeEditorFile(fileid, file)
}

// Sets editor file, creating model if not already created
export function changeEditorFile(fileid: string, file: File) {
	const uri = Monaco.Uri.file(fileid)
	const current = _editorInstance?.getModel()?.uri.path

	// do nothing if new file is same as current file
	if (current === uri.path) return
	let model = Monaco?.editor.getModel(uri)
	if (!model) model = Monaco?.editor.createModel(file.data, file.extension, uri)

	// store cached cursor position for this model
	if (current)
		_cachedPositions[current] = _editorInstance?.getPosition() ?? undefined
	const newPos = _cachedPositions[fileid]

	// set new model, set cursor positon from cache if available
	_editorInstance?.setModel(model)
	if (newPos) _editorInstance?.setPosition(newPos, 'Editor.changeEditorFile')
}

// Updates editor config based on user editor config
export function updateEditorConfig(config: UserEditorPrefs) {
	// monaco options
	const options: monaco.editor.IEditorOptions &
		monaco.editor.IGlobalEditorOptions = {
		fontSize: config.fontSize!,
		fontFamily: config.fontFamily!,
		lineNumbers: config.lineNumbers,
		minimap: {
			enabled: config.minimap
		}
	}
	_editorInstance?.updateOptions(options)

	if (!_vimMode && config.vimMode && _editorInstance) {
		_vimMode = MonacoVim?.initVimMode(_editorInstance, _statusEl, Statusbar)
	} else if (_vimMode && !config.vimMode) {
		_vimMode.dispose()
		_vimMode = undefined
	}
	setTimeout(clearFontCache, 10)
}

export function clearFontCache() {
	Monaco?.editor.remeasureFonts()
}

export function getModel(path: string): monaco.editor.ITextModel | null {
	return Monaco?.editor.getModel(Monaco.Uri.file(path))
}
export function getAllModels(): monaco.editor.ITextModel[] {
	return Monaco?.editor.getModels() ?? []
}

export const setTheme = (newTheme: string) => Monaco?.editor.setTheme(newTheme)
