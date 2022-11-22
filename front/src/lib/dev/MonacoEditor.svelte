<script lang="ts">
	import type { File, Layout, SupportedExtension, UserEditorPrefs } from '$common'
	import dark from '$core/monaco/dark'
	import light from '$core/monaco/light'
	import { wFiles, wLayout, wTheme, wUserEditorPrefs } from '$stores'
	import type { editor, Position } from 'monaco-editor'
	import { onMount } from 'svelte'
	import { get } from 'svelte/store'

	import Statusbar from '$core/monaco/statusbar'
	import * as wgsl from '$core/monaco/wgsl'
	import IconButton from '$lib/components/buttons/IconButton.svelte'
	import editorWorker from 'monaco-editor/esm/vs/editor/editor.worker?worker'
	import jsonWorker from 'monaco-editor/esm/vs/language/json/json.worker?worker'

	let divEl: HTMLDivElement
	let statusEl: HTMLDivElement

	var Monaco: typeof import('monaco-editor')
	/** @ts-ignore*/
	var MonacoVim: any
	let editorInstance: editor.IStandaloneCodeEditor | undefined = undefined
	let vimMode: any | undefined = undefined

	let cursorPosition: editor.ICursorPositionChangedEvent | null = null
	let currentExtension: SupportedExtension | null = null
	let cachedPositions: { [key: string]: Position | undefined } = {}

	// Store subscriptions
	wTheme.subscribe((newTheme) => Monaco?.editor.setTheme(newTheme))
	wUserEditorPrefs.subscribe(updateEditorConfig)
	wLayout.subscribe(changeFileFromLayout)

	// Initializes the monaco editor instance
	async function initEditor() {
		self.MonacoEnvironment = {
			getWorker(_, label) {
				if (label === 'json') {
					return new jsonWorker()
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
				highlightActiveIndentation: true,
				bracketPairsHorizontal: true
			},
			padding: {
				bottom: 20,
				top: 20
			}
		})

		// manually set theme and file on init
		Monaco.editor.setTheme(get(wTheme))
		changeFileFromLayout(get(wLayout))
		updateEditorConfig(get(wUserEditorPrefs))
		editorInstance.onDidChangeCursorPosition((ev) => {
			cursorPosition = ev
		})

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
		currentExtension = file?.extension ?? null
		if (file) changeEditorFile(fileid, file)
	}

	// Sets editor file, creating model if not already created
	function changeEditorFile(fileid: string, file: File) {
		const uri = Monaco.Uri.file(fileid)
		const current = editorInstance?.getModel()?.uri.path

		// do nothing if new file is same as current file
		if (current === uri.path) return
		let model = Monaco?.editor.getModel(uri)
		if (!model) model = Monaco?.editor.createModel(file.data, file.extension, uri)

		// override on change to target new file
		model.onDidChangeContent((ev) => {
			const content = editorInstance?.getModel()?.getValue()
			if (!content) return
			wFiles.writeFile(fileid, content)
		})

		// store cached cursor position for this model
		if (current) cachedPositions[current] = editorInstance?.getPosition() ?? undefined
		const newPos = cachedPositions[fileid]

		// set new model, set cursor positon from cache if available
		editorInstance?.setModel(model)
		if (newPos) editorInstance?.setPosition(newPos, 'Editor.changeEditorFile')
	}

	// Updates editor config based on user editor config
	function updateEditorConfig(config: UserEditorPrefs) {
		// monaco options
		const options: editor.IEditorOptions & editor.IGlobalEditorOptions = {
			fontSize: config.fontSize!,
			fontFamily: config.fontFamily!,
			lineNumbers: config.lineNumbers,
			minimap: {
				enabled: config.minimap
			}
		}
		editorInstance?.updateOptions(options)

		console.log(vimMode, config.vimMode)
		if (!vimMode && config.vimMode && editorInstance) {
			vimMode = MonacoVim?.initVimMode(editorInstance, statusEl, Statusbar)
		} else if (vimMode && !config.vimMode) {
			vimMode.dispose()
			vimMode = undefined
		}
		setTimeout(() => Monaco?.editor.remeasureFonts(), 10)
	}
	function clearFontCache() {
		Monaco?.editor.remeasureFonts()
	}

	onMount(initEditor)
</script>

<div id="container">
	<div id="editor-root" bind:this={divEl} />
	<div id="status-root" class:hide={editorInstance === undefined}>
		<div id="vim-status-root" bind:this={statusEl} />
		<div id="status-right">
			<IconButton size="xs" on:click={clearFontCache}>Clear</IconButton>
			<span
				>Ln {cursorPosition?.position.lineNumber ?? '?'}, Col {cursorPosition?.position.column ??
					'?'}</span
			>
			{#if currentExtension}
				<spam>{currentExtension}</spam>
			{/if}
		</div>
	</div>
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
		height: calc(100% - 24px);
		width: 100%;
	}
	#status-root {
		flex: 0 0 auto;
		height: 24px;
		width: 100%;
		display: flex;
		align-items: center;
		justify-content: space-between;
		font-size: var(--xxs);
		padding-inline: 6px;
		background-color: var(--background-alt);
		border-top: var(--border2);
		box-sizing: border-box;
		color: var(--text-accent-color);
		user-select: none;
	}
	#vim-status-root {
		flex: 0 0 auto;
		height: 24px;
		align-items: center;
		gap: 8px;
	}
	#status-right {
		align-items: center;
		display: flex;
		gap: 8px;
		min-width: max-content;
	}
	.hide {
		display: none;
	}
</style>
