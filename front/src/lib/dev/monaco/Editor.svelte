<script lang="ts">
	import { theme } from '$stores/theme'
	import type monaco from 'monaco-editor'
	import editorWorker from 'monaco-editor/esm/vs/editor/editor.worker?worker'
	import jsonWorker from 'monaco-editor/esm/vs/language/json/json.worker?worker'
	import { onMount } from 'svelte'
	import dark from './dark'
	import light from './light'

	import { wFiles } from '$stores/project'

	export let fileid: string
	$: file = $wFiles.map[fileid]

	let divEl: HTMLDivElement | null = null
	let editor: monaco.editor.IStandaloneCodeEditor | undefined = undefined
	let Monaco: any

	// $: {
	// 	editor?.setModel()
	// }

	onMount(async () => {
		// @ts-ignore
		self.MonacoEnvironment = {
			getWorker: function (_moduleId: any, label: string) {
				if (label === 'json') {
					return new jsonWorker()
				}
				return new editorWorker()
			}
		}
		Monaco = await import('monaco-editor')
		Monaco.editor.defineTheme('dark', dark)
		Monaco.editor.defineTheme('light', light)
		editor = Monaco.editor.create(divEl!, {
			value: file.data,
			language: 'wgsl',
			theme: 'dark',
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

		editor?.getModel()?.onDidChangeContent((ev) => {
			wFiles.update((files) => {
				let map = { ...files.map }
				map[fileid] = {
					...map[fileid],
					data: editor?.getModel()?.getValue() ?? ''
				}
				return {
					...files,
					map
				}
			})

			console.log($wFiles)
		})

		theme.subscribe((newTheme) => Monaco.editor.setTheme(newTheme))

		return () => {
			editor?.dispose()
		}
	})
</script>

{#if file}
	<div bind:this={divEl} class="h-screen" />
{:else}
	<div>No file</div>
{/if}

<style>
	.h-screen {
		height: 100%;
		width: 100%;
	}
</style>
