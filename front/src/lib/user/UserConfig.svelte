<script lang="ts">
	import IconButton from '$lib/components/buttons/IconButton.svelte'
	import {
		EDITOR_CONFIG_KEYS,
		GENERAL_CONFIG_KEYS,
		USER_CONFIG_META,
		type ConfigKey,
		type ConfigScope
	} from '$lib/consts/userConfig'
	import { toggleUserConfig, wUserConfigOpen } from '$stores/ui'
	import { setProperty, validate, wUserEditorConfig, wUserGeneralConfig } from '$stores/userConfig'

	let dialog: HTMLDialogElement
	let configGeneral = USER_CONFIG_META.general
	let configEditor = USER_CONFIG_META.editor

	$: {
		if (dialog) {
			if ($wUserConfigOpen) dialog.showModal()
			else dialog.close()
		}
	}
	function onChangeProp(scope: ConfigScope, key: ConfigKey, ev: Event) {
		const value = (ev.target as HTMLInputElement)?.value
		if (validate(scope, key, value)) setProperty(scope, key, value)
	}
</script>

<dialog bind:this={dialog}>
	<div class="modal">
		<div class="header">
			<h2 class="title">User config</h2>
			<IconButton on:click={toggleUserConfig} size="sm">Close</IconButton>
		</div>
		<div class="body">
			<div class="category-list">
				<div class="category">General</div>
				<div class="category">Editor</div>
				<div class="category">Keybinds</div>
				<div class="category">Theme</div>
			</div>
			<div class="category-body">
				<h3>General</h3>
				{#each GENERAL_CONFIG_KEYS as k}
					<div>
						<label for={k}><code>{k}</code></label>

						<p>
							{configGeneral[k].description}
						</p>
					</div>
					<div>
						<input
							id={k}
							type={configGeneral[k].type}
							value={$wUserGeneralConfig[k]}
							on:change={(ev) => onChangeProp('general', k, ev)}
							min={configGeneral[k].min}
							max={configGeneral[k].max}
						/>
						<span class="unit">{configGeneral[k].units} </span>
					</div>
				{/each}
				<h3>Editor</h3>
				{#each EDITOR_CONFIG_KEYS as k}
					<div>
						<label for={k}><code>{k}</code></label>

						<p>
							{configEditor[k].description}
						</p>
					</div>
					<div>
						<input
							id={k}
							type={configEditor[k].type}
							value={$wUserEditorConfig[k]}
							on:change={(ev) => onChangeProp('editor', k, ev)}
							min={configEditor[k].min}
							max={configEditor[k].max}
						/>
						<span class="unit">{configEditor[k].units ?? ''} </span>
					</div>
				{/each}
			</div>
		</div>
	</div>
</dialog>

<style>
	dialog {
		width: 50%;
		min-width: fit-content;
		max-width: 800px;
		padding: 0px;
		background-color: var(--transparent-high);
		backdrop-filter: blur(16px);
		border-radius: 6px;
	}
	.modal {
		display: flex;
		flex-direction: column;
		height: 100%;
	}
	.header {
		flex: 0 0 auto;
		display: flex;
		flex-direction: row;
		justify-content: space-between;
		align-items: center;
		padding-inline: 1rem;
		background-color: var(--transparent);
		height: 4rem;
		border-bottom: 1px solid var(--border-secondary);
	}
	.title {
		margin: 0;
	}
	.body {
		flex: 1 1 auto;
		display: flex;
		flex-direction: row;
	}
	.category-list {
		flex: 0 0 auto;
		border-right: 1px solid var(--border-secondary);
		padding-top: 1rem;
	}
	.category-body {
		background-color: var(--pure-bg);
		padding: 1rem;
		height: 100%;
		flex: 1 1 auto;
		display: grid;
		grid-template-columns: 2fr 1fr;
		align-items: center;
		gap: 1rem;
	}
	.category {
		color: var(--text-accent-color);
		font-size: var(--sm);
		cursor: pointer;
		padding: 4px;
		padding-inline: 1rem;
	}
	.unit {
		margin-left: 5px;
		color: var(--text-accent-color);
		font-size: var(--xs);
	}
	label {
		font-size: var(--md);
	}
	input {
		padding: 4px;
		outline: 1px solid var(--border-secondary);
	}
	p {
		margin-block: 4px;
		color: var(--text-accent-color);
		font-size: var(--xs);
	}
	h3 {
		grid-column: span 2;
		align-self: flex-start;
		margin: 0rem;
	}
</style>
