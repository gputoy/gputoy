<script lang="ts">
	import ConfigItem from '$lib/components/ConfigItem.svelte'
	import { EDITOR_CONFIG_KEYS, GENERAL_CONFIG_KEYS } from '$lib/consts/userConfig'
	import { wUserConfigOpen } from '$stores/ui'
	import { wUserEditorConfig, wUserGeneralConfig } from '$stores/userConfig'
	import Icon from 'svelte-awesome'
	import search from 'svelte-awesome/icons/search'
	import { fly } from 'svelte/transition'

	let category = 'general'
	let configSearch = ''
</script>

{#if $wUserConfigOpen}
	<div class="modal" transition:fly={{ x: 500, duration: 300 }}>
		<div class="body">
			<div class="category-list">
				<div class="category" class:category-active={category == 'general'}>General</div>
				<div class="category">Editor</div>
				<div class="category">Keybinds</div>
				<div class="category">Theme</div>
			</div>
			<div class="category-body">
				<input placeholder="Search" bind:value={configSearch} />
				<Icon data={search} class="search-icon" />
				{#each GENERAL_CONFIG_KEYS as k}
					{#if k.includes(configSearch)}
						<ConfigItem key={k} scope="general" value={$wUserGeneralConfig[k]} />
					{/if}
				{/each}
				{#each EDITOR_CONFIG_KEYS as k}
					{#if k.includes(configSearch)}
						<ConfigItem key={k} scope="editor" value={$wUserEditorConfig[k]} />
					{/if}
				{/each}
			</div>
		</div>
	</div>
{/if}

<style>
	.modal {
		position: absolute;
		z-index: var(--z-modal);
		right: 0;
		border-left: 1px solid var(--border-primary);
		width: 20%;
		min-width: fit-content;
		height: 100%;
	}

	.body {
		flex: 1 1 auto;
		display: flex;
		flex-direction: row;
		height: 100%;
		background-color: var(--pure-bg);
	}
	.category-list {
		flex: 0 0 auto;
		border-right: 1px solid var(--border-secondary);
		padding-top: 1rem;
		background-color: var(--tertiary-color);
	}
	.category-body {
		padding: 1rem;
		display: flex;
		flex-direction: column;
		width: 100%;
		align-items: left;
		gap: 1rem;
	}
	.category {
		color: var(--border-color);
		font-weight: 600;
		font-size: var(--xs);
		cursor: pointer;
		padding: 4px;
		padding-inline: 1rem;
	}
	.category:hover {
		background-color: var(--glass-med);
	}
	.category-active {
		background-color: var(--glass-low);
	}
	input {
		padding-left: 24px;
	}
</style>
