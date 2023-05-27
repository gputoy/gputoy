<script lang="ts">
	import { wTheme } from '$stores'
	import { onMount } from 'svelte'
	import { get } from 'svelte/store'

	import Icon from '$lib/components/Icon.svelte'

	import { dLayout, getLayout } from '$core/layout'
	import { dPreferences } from '$core/preferences'
	import * as monaco from '$monaco'

	let divEl: HTMLDivElement
	let statusEl: HTMLDivElement

	// Store subscriptions
	wTheme.subscribe(monaco.setTheme)
	dPreferences.subscribe(monaco.updateEditorConfig)
	dLayout.subscribe(monaco.changeFileFromLayout)

	const { cursorPosition, currentExtension } = monaco

	onMount(() =>
		monaco.initEditor(divEl, statusEl, {
			theme: get(wTheme),
			layout: getLayout(),
			prefs: get(dPreferences)
		})
	)
</script>

<div id="container">
	<div class="bottom-flat" id="editor-root" bind:this={divEl} />
	<div id="status-root" class="bottom-round" class:hide={monaco.noInstance}>
		<div id="vim-status-root" bind:this={statusEl} />
		<div id="status-right">
			<span>
				Ln {$cursorPosition?.position.lineNumber ?? '?'}, Col {$cursorPosition
					?.position.column ?? '?'}
			</span>
			<div class="analyzer-result">
				<span> 2 </span>
				<Icon class="sm" name="alert-circle" stroked />
				<span> 0 </span>
				<Icon class="sm" name="alert-triangle" stroked />
			</div>
			<button class="xs clear" on:click={monaco.clearFontCache}>
				<Icon name="refresh-ccw" stroked thick />
			</button>
			{#if $currentExtension}
				<spam>{$currentExtension}</spam>
			{/if}
		</div>
	</div>
</div>

<style>
	#container {
		width: 100%;
		display: flex;
		flex: 1 1 auto;
		flex-direction: column;
		max-height: 100%;
	}
	#editor-root {
		flex: 1 1 auto;
		height: calc(100% - calc(var(--xs-nav) + var(--gap2)));
		width: 100%;
		overflow: hidden;
	}
	#status-root {
		flex: 0 0 auto;
		height: var(--xs-nav);
		font-size: var(--xs);
		width: 100%;
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding-inline: 12px;
		background-color: var(--background-menu);
		color: var(--text-accent-color);
		user-select: none;
		box-sizing: border-box;
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
