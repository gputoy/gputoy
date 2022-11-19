<script lang="ts">
	import IconButton from '$lib/components/buttons/IconButton.svelte'
	import FileIcon from '$lib/components/FileIcon.svelte'
	import Icon from '$lib/components/Icon.svelte'
	import Logo from '$lib/components/Logo.svelte'
	import { wFiles, wLayout } from '$stores/project'
	import Editor from '../monaco/Editor.svelte'

	$: workspace = $wLayout.workspace
	$: fileindex = $wLayout.fileIndex ?? null
	$: fileid = fileindex != null ? workspace[fileindex] : null

	$: highlight = workspace.map((v) => false)

	function getFile(fileid: string) {
		return $wFiles.map[fileid]
	}

	function getCanonicalName(fileid: string) {
		let file = getFile(fileid)
		return file.fileName + '.' + file.extension
	}

	function handleClick(ev: MouseEvent, index: number) {
		// middle click
		if (ev.button == 1) {
			wLayout.closeWorkspaceFile(index)
			ev.preventDefault()
			return
		}
		let fileid = workspace[index]
		if (fileid) wLayout.update((layout) => ({ ...layout, fileIndex: index }))
	}
	function handleClose(index: number) {
		wLayout.closeWorkspaceFile(index)
	}
	function handleMouseEnter(index: number) {
		highlight[index] = true
	}
	function handleMouseLeave(index: number) {
		highlight[index] = false
	}
</script>

<div class="editor-container">
	{#if fileid}
		<div class="file-tabs">
			{#each workspace as fileid, i}
				<div
					class="file-tab"
					class:selected={i == fileindex}
					on:mouseenter={() => handleMouseEnter(i)}
					on:mouseleave={() => handleMouseLeave(i)}
					on:mousedown={(ev) => handleClick(ev, i)}
				>
					<FileIcon extension={$wFiles.map[fileid].extension} size={14} />
					<span>
						{getCanonicalName(fileid)}
					</span>
					<IconButton empty size="xs" on:click={() => handleClose(i)}>
						<Icon
							stroked
							thick
							name="x"
							style="visibility:{highlight[i] ? 'visible' : 'hidden'};"
						/>
					</IconButton>
				</div>
			{/each}
			<div class="filler" />
		</div>
		<Editor />
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
		padding-left: 0.4rem;
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
	span {
		margin-left: 0.4rem;
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
</style>
