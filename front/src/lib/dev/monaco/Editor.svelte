<script lang="ts">
	import { wFiles, wLayout } from '$stores/project'
	import { theme } from '$stores/theme'
	import { wUserEditorConfig } from '$stores/userConfig'
	import type { editor, Position } from 'monaco-editor'
	import type { File, Layout, UserEditorConfig } from 'src/generated/types'
	import { onMount } from 'svelte'
	import { get } from 'svelte/store'
	import dark from './dark'
	import light from './light'

	import editorWorker from 'monaco-editor/esm/vs/editor/editor.worker?worker'
	import cssWorker from 'monaco-editor/esm/vs/language/css/css.worker?worker'
	import htmlWorker from 'monaco-editor/esm/vs/language/html/html.worker?worker'
	import jsonWorker from 'monaco-editor/esm/vs/language/json/json.worker?worker'
	import tsWorker from 'monaco-editor/esm/vs/language/typescript/ts.worker?worker'
	import * as wgsl from './wgsl'

	let divEl: HTMLDivElement
	let statusEl: HTMLDivElement

	var Monaco: typeof import('monaco-editor')
	/** @ts-ignore*/
	var MonacoVim: any
	let editorInstance: editor.IStandaloneCodeEditor | undefined = undefined
	let vimMode: any | undefined = undefined

	let cachedPositions: { [key: string]: Position | undefined } = {}

	// Store subscriptions
	theme.subscribe((newTheme) => Monaco?.editor.setTheme(newTheme))
	wUserEditorConfig.subscribe(updateEditorConfig)
	wLayout.subscribe(changeFileFromLayout)

	// Initializes the monaco editor instance
	async function initEditor() {
		self.MonacoEnvironment = {
			getWorker(_, label) {
				if (label === 'json') {
					return new jsonWorker()
				}
				if (label === 'css' || label === 'scss' || label === 'less') {
					return new cssWorker()
				}
				if (label === 'html' || label === 'handlebars' || label === 'razor') {
					return new htmlWorker()
				}
				if (label === 'typescript' || label === 'javascript') {
					return new tsWorker()
				}
				return new editorWorker()
			}
		}
		Monaco = await import('monaco-editor')
		/** @ts-ignore */
		MonacoVim = await import('monaco-vim')
		Monaco.editor.defineTheme('dark', dark)
		Monaco.editor.defineTheme('light', light)

		Monaco.languages.register(wgsl.extensionPoint)
		Monaco.languages.onLanguage(wgsl.id, () => {
			Monaco.languages.setMonarchTokensProvider(wgsl.id, wgsl.monarch)
			Monaco.languages.setLanguageConfiguration(wgsl.id, wgsl.config)
		})

		editorInstance = Monaco.editor.create(divEl, {
			automaticLayout: true,
			minimap: {
				enabled: false
			},
			guides: {
				highlightActiveBracketPair: true,
				bracketPairs: true,
				indentation: true,
				highlightActiveIndentation: true
			},
			padding: {
				bottom: 10,
				top: 20
			}
		})

		// manually set theme and file on init
		Monaco.editor.setTheme(get(theme))
		changeFileFromLayout(get(wLayout))
		updateEditorConfig(get(wUserEditorConfig))

		// On destroy, dispose editor
		return () => {
			editorInstance?.dispose()
		}
	}

	// Finds which file to display based on layout
	function changeFileFromLayout(layout: Layout) {
		if (!Monaco) return
		if (layout.fileIndex == null) return
		const fileid = layout.workspace[layout.fileIndex]
		if (!fileid) return
		const file = wFiles.getFile(fileid)
		if (file) changeEditorFile(fileid, file)
	}

	// Sets editor file, creating model if not already created
	function changeEditorFile(fileid: string, file: File) {
		const uri = Monaco.Uri.file(fileid)
		const prev = editorInstance?.getModel()?.uri.path
		if (prev === uri.path) return
		let model = Monaco?.editor.getModel(uri)
		if (!model) {
			model = Monaco?.editor.createModel(file.data, file.extension, uri)
		}
		model.onDidChangeContent((ev) => {
			const content = editorInstance?.getModel()?.getValue()
			if (!content) return
			wFiles.writeFile(fileid, content)
		})
		if (prev) cachedPositions[prev] = editorInstance?.getPosition() ?? undefined
		const newPos = cachedPositions[fileid]
		editorInstance?.setModel(model)
		if (newPos) editorInstance?.setPosition(newPos, 'Editor.changeEditorFile')
	}

	// Updates editor config based on user editor config
	function updateEditorConfig(config: UserEditorConfig) {
		const options: editor.IEditorOptions & editor.IGlobalEditorOptions = {
			fontSize: config.fontSize!,
			fontFamily: config.fontFamily!,
			lineNumbers: config.lineNumbers,
			minimap: {
				enabled: config.minimap
			}
		}
		editorInstance?.updateOptions(options)
		Monaco?.editor.remeasureFonts()
		console.log('in updateEditorConfig: ', vimMode, config.vimMode, editorInstance !== undefined)

		if (!vimMode && config.vimMode && editorInstance) {
			console.log('monaco vim:', MonacoVim)

			vimMode = MonacoVim?.initVimMode(editorInstance, statusEl)
		} else if (vimMode && !config.vimMode) {
			vimMode.dispose()
			vimMode = undefined
		}
	}

	onMount(initEditor)
</script>

<div id="container">
	<div id="editor-root" bind:this={divEl} />
	<div id="status-root" bind:this={statusEl} />
</div>

<style>
	#container {
		width: 100%;
		height: calc(100% - 32px);
		display: flex;
		flex: 1 1 auto;
		flex-direction: column;
	}
	#editor-root {
		flex: 1 1 auto;
		height: calc(100% - 34px);
		width: 100%;
	}
	#status-root {
		flex: 0 0 auto;
		height: 32px;
		width: 100%;
		display: flex;
		align-items: center;
		justify-content: space-between;
		font-size: var(--xs);
		padding-inline: 6px;
		background-color: var(--background-alt);
		border-top: var(--border2);
		box-sizing: border-box;
	}
</style>
