<script lang="ts">
	import { getCanonicalName } from '$core/files'
	import { closeTab, rTabIndex, rTabs, setFileIndex } from '$core/layout'
	import FileIcon from '$lib/components/file/FileIcon.svelte'
	import Icon from '$lib/components/Icon.svelte'
	import Logo from '$lib/components/Logo.svelte'
	import MonacoEditor from '$lib/workspace/MonacoEditor.svelte'
	import { wFiles, wModelDirty } from '$stores'

	$: fileid = $rTabIndex != null ? $rTabs[$rTabIndex] : null
	function handleClick(ev: MouseEvent, index: number) {
		// middle click
		if (ev.button == 1) {
			closeTab(index)
			ev.preventDefault()
			closeTab
		}
		if ($rTabs[index]) setFileIndex(index)
	}
	function handleClose(index: number) {
		closeTab(index)
	}
</script>

<div class="editor-container pane">
	{#if fileid}
		<div class="file-tabs">
			{#each $rTabs as fileid, i}
				<div
					class="file-tab"
					class:selected={i == $rTabIndex}
					on:mousedown={(ev) => handleClick(ev, i)}
				>
					<FileIcon extension={$wFiles.map[fileid].extension} size={13} />
					<span>
						{getCanonicalName(fileid)}
					</span>
					{#if $wModelDirty[fileid] ?? false}
						<Icon thick name="circle" class="xs" />
					{/if}
					<button class="xs clear" on:click={() => handleClose(i)}>
						<Icon stroked thick name="x" />
					</button>
				</div>
			{/each}
			<div class="filler" />
		</div>
		<MonacoEditor />
	{:else}
		<div class="editor-helper">
			<Logo size="100px" />
			<h2>Choose a file to get started!</h2>
		</div>
	{/if}
</div>

<style>
	.file-tabs {
		height: 2rem;
		display: flex;
		flex: 0 0 auto;
		background-color: var(--border-primary);
		gap: var(--border-primary-width);
	}

	.editor-container {
		height: 100%;
		display: flex;
		flex-direction: column;
		box-sizing: border-box;
		border-left: var(--border);
	}

	.file-tab {
		font-size: var(--sm);
		padding: 4px;
		padding-left: 10px;
		gap: 4px;
		cursor: pointer;
		height: 100%;
		width: fit-content;
		text-overflow: ellipsis;
		white-space: nowrap;
		max-width: 10rem;
		display: flex;
		box-sizing: border-box;
		justify-content: center;
		align-items: center;
		user-select: none;
		transition: none;
	}

	.editor-helper {
		width: 100%;
		height: 100%;
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
		background-color: var(--border-primary);
	}

	.selected {
		background-color: var(--background-alt);
		color: var(--text-important);
		border-top-left-radius: var(--pane-radius);
		border-top-right-radius: var(--pane-radius);
	}

	:global(.file-tab:hover .x-button) {
		visibility: visible;
	}
	:global(.file-tab .x-button) {
		visibility: hidden;
	}
</style>
