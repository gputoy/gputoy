<script lang="ts">
	import type { FileTreeNode } from '$core/files'
	import FileNode from './FileNode.svelte'
	import NodeEntry from './NodeEntry.svelte'

	export let node: FileTreeNode
	export let open = false
	export let foldLines = false
	export let clazz = ''
</script>

{#if open}
	<ul class:foldLines>
		{#each Object.values(node.children) as child}
			{#if 'children' in child}
				<!-- Branch -->
				<FileNode fileNode={child} />
			{:else if 'id' in child}
				<!-- Leaf -->
				<NodeEntry node={child} {clazz} />
			{/if}
		{/each}
	</ul>
{/if}

<style>
	ul {
		margin-top: var(--spacing);
		display: flex;
		flex-direction: column;
		padding-left: var(--indent);
		list-style: none;
		gap: var(--spacing);
		font-size: var(--xs);
	}
	.foldLines {
		position: relative;
	}
	.foldLines:before {
		position: absolute;
		content: '';
		background-color: var(--border-primary);
		width: 1px;
		height: 100%;
	}
</style>
