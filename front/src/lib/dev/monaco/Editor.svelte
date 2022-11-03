<script lang="ts">
	import { wFiles, wLayout } from '$stores/project'
	import { theme } from '$stores/theme'
	import { wUserEditorConfig } from '$stores/userConfig'
	import type { editor } from 'monaco-editor'
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

	let divEl: HTMLDivElement

	/** @type {typeof import('monaco-editor')} */
	var Monaco: typeof import('monaco-editor')
	let editorInstance: editor.IStandaloneCodeEditor | undefined = undefined

	// Store subscriptions
	theme.subscribe((newTheme) => Monaco?.editor.setTheme(newTheme))
	wUserEditorConfig.subscribe(updateEditorConfig)
	wLayout.subscribe(changeFileFromLayout)

	// Initializes the monaco editor instance
	async function initEditor() {
		self.MonacoEnvironment = {
			getWorker(_, label) {
				console.log('worker label: ', label)
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
		Monaco.editor.defineTheme('dark', dark)
		Monaco.editor.defineTheme('light', light)

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
				top: 10
			}
		})

		// manually set theme and file on init
		Monaco.editor.setTheme(get(theme))
		changeFileFromLayout(get(wLayout))

		// On destroy, dispose editor
		return () => {
			console.log('Disposing monaco editor')
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
		if (editorInstance?.getModel()?.uri.path === uri.path) return
		let model = Monaco?.editor.getModel(uri)
		if (!model) {
			model = Monaco?.editor.createModel(file.data, file.extension, uri)
		}
		model.onDidChangeContent((ev) => {
			const content = editorInstance?.getModel()?.getValue()
			if (!content) return
			wFiles.writeFile(fileid, content)
		})
		editorInstance?.setModel(model)
	}

	// Updates editor config based on user editor config
	function updateEditorConfig(config: UserEditorConfig) {
		const options: editor.IEditorOptions & editor.IGlobalEditorOptions = {
			fontSize: config.fontSize!,
			fontFamily: config.fontFamily!
		}
		editorInstance?.updateOptions(options)
	}

	onMount(initEditor)
</script>

<div id="editor-root" bind:this={divEl} class="h-screen" />

<style>
	.h-screen {
		height: 100%;
		width: 100%;
	}
</style>
