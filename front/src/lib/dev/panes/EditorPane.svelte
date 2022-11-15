<script lang="ts">
	import Logo from '$lib/components/Logo.svelte'
	import { wFiles, wLayout } from '$stores/project'
	import Editor from '../monaco/Editor.svelte'

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
</script>

<div class="file-tabs">
	{#each workspace as fileid, i}
		<div class="file-tab" class:selected={i == fileindex} on:click={() => setWorkspaceIndex(i)}>
			{getCanonicalName(fileid)}
		</div>
	{/each}
</div>
{#if fileid}
	<Editor />
{:else}
	<div class="editor-helper">
		<Logo size="100px" fill="var(--glass-med)" />
		<h2 style="color: var(--glass-med)">Choose a file to get started!</h2>
	</div>
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
	}
</style>
