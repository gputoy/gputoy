<script lang="ts">
	import { wLayout, wTheme, wUserEditorPrefs } from '$stores'
	import { onMount } from 'svelte'
	import { get } from 'svelte/store'

	import IconButton from '$lib/components/buttons/IconButton.svelte'
	import Icon from '$lib/components/Icon.svelte'

	import * as monaco from '$monaco'

	let divEl: HTMLDivElement
	let statusEl: HTMLDivElement

	// Store subscriptions
	wTheme.subscribe(monaco.setTheme)
	wUserEditorPrefs.subscribe(monaco.updateEditorConfig)
	wLayout.subscribe(monaco.changeFileFromLayout)

	const { cursorPosition, currentExtension } = monaco

	onMount(() =>
		monaco.initEditor(divEl, statusEl, {
			theme: get(wTheme),
			layout: get(wLayout),
			prefs: get(wUserEditorPrefs)
		})
	)
</script>

<div id="container">
	<div id="editor-root" bind:this={divEl} />
	<div id="status-root" class:hide={monaco.noInstance}>
		<div id="vim-status-root" bind:this={statusEl} />
		<div id="status-right">
			<span>
				Ln {$cursorPosition?.position.lineNumber ?? '?'}, Col {$cursorPosition
					?.position.column ?? '?'}
			</span>
			<div class="analyzer-result">
				<span> 2 </span>
				<Icon name="alert-circle" stroked />
				<span> 0 </span>
				<Icon name="alert-triangle" stroked />
			</div>
			<IconButton size="xs" on:click={monaco.clearFontCache} empty>
				<Icon name="refresh-ccw" stroked thick />
			</IconButton>
			{#if $currentExtension}
				<spam>{$currentExtension}</spam>
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
		padding-inline: 12px;
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
		gap: 12px;
		min-width: max-content;
	}
	.hide {
		display: none;
	}
	.analyzer-result {
		display: flex;
		align-items: center;
		gap: 4px;
	}
</style>
