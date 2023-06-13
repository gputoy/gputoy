import type { File, Layout, Preferences, SupportedExtension } from '$gen'
import type * as monaco from 'monaco-editor'

import editorWorker from 'monaco-editor/esm/vs/editor/editor.worker?worker'
import jsonWorker from 'monaco-editor/esm/vs/language/json/json.worker?worker'
import gputWorker from './wgsl/worker?worker'

import type { Theme } from '$core/theme'
import { wFiles } from '$stores'
import { writable } from 'svelte/store'
import genDark from './dark'
import setJSONSchema from './json'
import genLight from './light'
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
	prefs: Preferences
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
	setJSONSchema(Monaco)

	_editorInstance = Monaco.editor.create(divEl, {
		automaticLayout: true,
		overviewRulerBorder: false,
		padding: {
			bottom: 20,
			top: 20
		},
		experimentalWhitespaceRendering: 'font'
	})

	// manually set theme and file on init
	setTheme(init.theme)
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
	if (layout.tabIndex == null) return
	const fileid = layout.tabs[layout.tabIndex]
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
export function updateEditorConfig(preferences: Preferences) {
	// monaco options
	const options: monaco.editor.IEditorOptions &
		monaco.editor.IGlobalEditorOptions = {
		fontSize: preferences.editor['font-size'],
		fontFamily: preferences.editor['font'],
		fontLigatures: preferences.editor['font-ligatures'],
		lineNumbers: preferences.editor['line-numbers'],
		cursorStyle: preferences.editor['cursor-style'],
		autoIndent: preferences.editor['auto-indent'],
		smoothScrolling: preferences.editor['smooth-scrolling'],
		cursorBlinking: preferences.editor['cursor-blinking'],
		wordWrap: preferences.editor['word-wrap'] ? 'on' : 'off',
		cursorSmoothCaretAnimation: preferences.editor['smooth-caret']
			? 'on'
			: 'off',
		scrollBeyondLastLine: preferences.editor['scroll-beyond-last-line'],
		minimap: {
			enabled: preferences.editor.minimap
		},
		guides: {
			highlightActiveBracketPair:
				preferences.editor.guides['highlight-active-bracket-pair'],
			bracketPairs: preferences.editor.guides['bracket-pairs'],
			indentation: preferences.editor.guides.indentation,
			highlightActiveIndentation:
				preferences.editor.guides['highlight-active-indentation'],
			bracketPairsHorizontal:
				preferences.editor.guides['bracket-pairs-horizontal']
		}
	}
	_editorInstance?.updateOptions(options)

	const vimMode = preferences.editor['vim-mode']
	if (!_vimMode && vimMode.enabled && _editorInstance) {
		_vimMode = MonacoVim?.initVimMode(_editorInstance, _statusEl, Statusbar)
	} else if (_vimMode && !vimMode.enabled) {
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

var definedThemes: { [key: string]: boolean } = {}
const genThemes: { [key: string]: () => monaco.editor.IStandaloneThemeData } = {
	light: genLight,
	dark: genDark
}
export const setTheme = (newTheme: string) => {
	// the store subscription will try setting the theme before monaco is loaded
	if (!Monaco) return
	if (!definedThemes[newTheme]) {
		const gen = genThemes[newTheme]
		if (!newTheme) return
		const theme = gen()
		Monaco?.editor.defineTheme(newTheme, theme)
		definedThemes[newTheme] = true
	}
	Monaco?.editor.setTheme(newTheme)
}
