<script lang="ts">
	import Accordian from '$lib/components/Accordian.svelte'
	import IconButton from '$lib/components/buttons/IconButton.svelte'
	import FileNode from '$lib/components/FileNode.svelte'
	import Icon from '$lib/components/Icon.svelte'
	import type { FileTreeNode } from '$lib/core/fileTree'
	import { wFiles, wProjectMeta } from '$stores/project'
	let root: FileTreeNode | undefined
	$: {
		root = wFiles.buildTree()
	}
</script>

<div id="pane-root">
	<Accordian title="Summary">
		<svelte:fragment slot="menu">
			<IconButton empty>
				<Icon stroked name="edit-2" />
			</IconButton>
		</svelte:fragment>
		<svelte:fragment slot="content">
			<h2 class="title">{$wProjectMeta.title}</h2>
			<p class="desc">
				{$wProjectMeta.description}
			</p>
		</svelte:fragment>
	</Accordian>
	<Accordian title="Files">
		<svelte:fragment slot="menu">
			<IconButton empty>
				<Icon stroked name="file-plus" />
			</IconButton>
			<IconButton empty>
				<Icon stroked name="folder-plus" />
			</IconButton>
		</svelte:fragment>
		<svelte:fragment slot="content">
			{#if root}
				<FileNode fileNode={root} />
			{/if}
		</svelte:fragment>
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
		margin-block: 0rem;
		font-size: var(--xl);
	}
	.desc {
		font-size: var(--xs);
	}
</style>
