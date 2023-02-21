<script lang="ts">
	import { EDITOR_CONFIG_KEYS, GENERAL_CONFIG_KEYS } from '$core/consts'
	import ConfigItem from '$lib/components/ConfigItem.svelte'
	import Icon from '$lib/components/Icon.svelte'
	import { wUserEditorPrefs, wUserGeneralPrefs, wUserPrefsOpen } from '$stores'
	import { fly } from 'svelte/transition'

	const lowerGeneralKeys: string[] = GENERAL_CONFIG_KEYS.map((s) =>
		s.toLowerCase()
	)
	const lowerEditorKeys: string[] = EDITOR_CONFIG_KEYS.map((s) =>
		s.toLowerCase()
	)

	let category = 'general'
	let configSearch = ''
	$: searchLower = configSearch.toLowerCase()
</script>

{#if $wUserPrefsOpen}
	<div class="modal" transition:fly={{ x: 500, duration: 300 }}>
		<div class="body">
			<div class="category-list">
				<div class="category" class:category-active={category == 'general'}>
					General
				</div>
				<div class="category">Editor</div>
				<div class="category">Keybinds</div>
				<div class="category">Theme</div>
			</div>
			<div class="category-body">
				<input placeholder="Search" bind:value={configSearch} />
				<Icon
					name="search"
					stroked
					size="1em"
					style="position:absolute;translate: 3px 3px;"
				/>
				{#each GENERAL_CONFIG_KEYS as k, i}
					{#if lowerGeneralKeys[i].includes(searchLower)}
						<ConfigItem key={k} scope="general" value={$wUserGeneralPrefs[k]} />
					{/if}
				{/each}
				{#each EDITOR_CONFIG_KEYS as k, i}
					{#if lowerEditorKeys[i].includes(searchLower)}
						<ConfigItem key={k} scope="editor" value={$wUserEditorPrefs[k]} />
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
		border-left: var(--border);
		width: 20%;
		min-width: fit-content;
		max-width: 50%;
		width: 400px;
		height: 100%;
		box-shadow: var(--box-shadow);
	}

	.body {
		flex: 1 1 auto;
		display: flex;
		flex-direction: row;
		height: 100%;
		background-color: var(--background-content);
	}
	.category-list {
		flex: 0 0 auto;
		border-right: var(--border2);
		padding-top: 1rem;
		background-color: var(--background-splash);
	}
	.category-body {
		height: 100%;
		padding: 1rem;
		display: flex;
		flex-direction: column;
		width: 100%;
		align-items: left;
		gap: 1rem;
		overflow: scroll;
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
