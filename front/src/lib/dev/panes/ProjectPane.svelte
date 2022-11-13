<script lang="ts">
	import Accordian from '$lib/components/Accordian.svelte'
	import FileNode from '$lib/components/FileNode.svelte'
	import type { FileTreeNode } from '$lib/core/fileTree'
	import { dCanModifyProject, wFiles, wProjectMeta } from '$stores/project'
	let root: FileTreeNode | undefined
	$: {
		root = wFiles.buildTree()
	}
</script>

<div id="pane-root">
	<Accordian title="Summary">
		{#if $dCanModifyProject}
			<input class="title clear" type="text" bind:value={$wProjectMeta.title} />
			<textarea class="desc clear" type="text" bind:value={$wProjectMeta.description} />
		{:else}
			<h1 class="title">{$wProjectMeta.title}</h1>
			<p class="desc">
				{$wProjectMeta.description}
			</p>
		{/if}
	</Accordian>
	<Accordian title="Files">
		{#if root}
			<FileNode fileNode={root} />
		{/if}
	</Accordian>
</div>

<style>
	#pane-root {
		display: flex;
		flex-direction: column;
		width: 100%;
		height: 100%;
		background-color: var(--background-content);
		margin: 0;
		border: 1px transparent solid;
	}

	.title {
		font-size: var(--xl);
		margin: 8px;
		padding: 8px;
	}

	.clear {
		background-color: transparent;
		margin-left: 4px;
	}

	textarea {
		align-self: center;
	}
</style>
