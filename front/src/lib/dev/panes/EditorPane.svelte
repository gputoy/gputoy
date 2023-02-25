<script lang="ts">
	import { getCanonicalName } from '$core/files'
	import {
		closeWorkspaceFile,
		rFileIndex,
		rWorkspace,
		setFileIndex
	} from '$core/layout'
	import IconButton from '$lib/components/buttons/IconButton.svelte'
	import FileIcon from '$lib/components/file/FileIcon.svelte'
	import Icon from '$lib/components/Icon.svelte'
	import Logo from '$lib/components/Logo.svelte'
	import MonacoEditor from '$lib/dev/MonacoEditor.svelte'
	import { wFiles, wModelDirty } from '$stores'

	// let Editor: any
	$: fileid = $rFileIndex != null ? $rWorkspace[$rFileIndex] : null

	function handleClick(ev: MouseEvent, index: number) {
		// middle click
		if (ev.button == 1) {
			closeWorkspaceFile(index)
			ev.preventDefault()
			return
		}
		if ($rWorkspace[index]) setFileIndex(index)
	}
	function handleClose(index: number) {
		closeWorkspaceFile(index)
	}
</script>

<div class="editor-container">
	{#if fileid}
		<div class="file-tabs">
			{#each $rWorkspace as fileid, i}
				<div
					class="file-tab"
					class:selected={i == $rFileIndex}
					on:mousedown={(ev) => handleClick(ev, i)}
				>
					<FileIcon extension={$wFiles.map[fileid].extension} size={13} />
					<span>
						{getCanonicalName(fileid)}
					</span>
					{#if $wModelDirty[fileid] ?? false}
						<Icon thick name="circle" size="0.75em" />
					{/if}
					<IconButton empty size="xs" on:click={() => handleClose(i)}>
						<Icon stroked thick name="x" clazz="x-button" />
					</IconButton>
				</div>
			{/each}
			<div class="filler" />
		</div>
		<MonacoEditor />
	{:else}
		<div class="editor-helper">
			<Logo size="100px" fill="var(--glass-med)" />
			<h2 style="color: var(--glass-med)">Choose a file to get started!</h2>
		</div>
	{/if}
</div>

<style>
	.file-tabs {
		height: 2rem;
		display: flex;
		flex: 0 0 auto;
	}

	.editor-container {
		height: 100%;
		display: flex;
		flex-direction: column;
	}

	.file-tab {
		font-size: var(--xs);
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
		border-bottom: var(--border2);
		border-right: var(--border2);
		user-select: none;
		transition: none;
	}
	.filler {
		flex: 1 1 auto;
		border-bottom: var(--border2);
	}

	.editor-helper {
		width: 100%;
		height: 100%;
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
	}

	.selected {
		background-color: var(--background-alt);
		border-bottom: 1px solid transparent;
		color: var(--text-important);
	}

	:global(.file-tab:hover .x-button) {
		visibility: visible;
	}
	:global(.file-tab .x-button) {
		visibility: hidden;
	}
</style>
