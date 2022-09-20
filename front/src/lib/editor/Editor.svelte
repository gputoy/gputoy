<script lang="ts">
	import type monaco from 'monaco-editor';
	import editorWorker from 'monaco-editor/esm/vs/editor/editor.worker?worker';
	import jsonWorker from 'monaco-editor/esm/vs/language/json/json.worker?worker';
	import { onMount } from 'svelte';
	import dark from './dark';

	let divEl: HTMLDivElement = null;
	let editor: monaco.editor.IStandaloneCodeEditor = undefined;
	let Monaco;

	onMount(async () => {
		console.log(editor);
		// @ts-ignore
		self.MonacoEnvironment = {
			getWorker: function (_moduleId: any, label: string) {
				if (label === 'json') {
					return new jsonWorker();
				}
				return new editorWorker();
			}
		};
		Monaco = await import('monaco-editor');
		Monaco.editor.defineTheme('dark', dark);
		editor = Monaco.editor.create(divEl, {
			value: ['@fragment', 'fn main() -> vec4<f32> {', '\treturn vec4<f32>();', '}'].join('\n'),
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
		});
		return () => {
			editor.dispose();
		};
	});
</script>

<div bind:this={divEl} class="h-screen" />

<style>
	.h-screen {
		height: 100%;
		width: 100%;
	}
</style>
