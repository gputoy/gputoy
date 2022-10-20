<script lang="ts">
	import { wFiles, wLayout } from '$stores/project'
	import Editor from './monaco/Editor.svelte'

	$: workspace = $wLayout.workspace
	$: fileindex = $wLayout.fileIndex ?? null
	$: fileid = fileindex != null ? workspace[fileindex] : null

	function getFile(fileid: string) {
		return $wFiles.map[fileid]
	}

	function getCanonicalName(fileid: string) {
		let file = getFile(fileid)
		return file.fileName + '.' + file.extension
	}

	function setWorkspaceIndex(index: number) {
		let fileid = workspace[index]
		if (fileid) wLayout.update((layout) => ({ ...layout, fileIndex: index }))
	}

	$: console.log('fileid: ', fileid)
</script>

<div class="file-tabs">
	{#each workspace as fileid, i}
		<div class="file-tab" class:selected={i == fileindex} on:click={() => setWorkspaceIndex(i)}>
			{getCanonicalName(fileid)}
		</div>
	{/each}
</div>
{#if fileid}
	<Editor {fileid} />
{:else}
	<div>No file selected</div>
{/if}

<style>
	.file-tabs {
		height: 2rem;
		display: flex;
	}

	.file-tab {
		font-size: var(--sm);
		padding: 0.5rem;
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
	}

	.selected {
		background-color: var(--primary-color);
	}
</style>
