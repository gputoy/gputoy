<script lang="ts">
	import type { FileTreeNode } from '$core/files'
	import { wLayout } from '$stores'
	import DirectoryContents from './DirectoryContents.svelte'
	import NodeEntry from './NodeEntry.svelte'

	export let fileNode: FileTreeNode
	$: open = $wLayout.fileTreeState[fileNode.absoluteDir]?.open ?? false
</script>

{#if fileNode.dir == ''}
	<!-- Root -->
	<div class="dir">
		<DirectoryContents node={fileNode} clazz="node-entry" open />
	</div>
{:else}
	<li class="dir">
		<NodeEntry node={fileNode} clazz="node-entry" {open} />
		<DirectoryContents node={fileNode} clazz="node-entry" foldLines {open} />
	</li>
{/if}

<style>
	:global(.node-entry) {
		width: 100%;
		height: 1rem;
		font-size: var(--xs);
		background: transparent;
		color: var(--text-color);
		border: none;
		display: flex;
		justify-content: space-between;
		align-items: center;
		cursor: pointer;
		position: relative;
		gap: 8px;
		user-select: none;
	}
	:global(.node-entry:before) {
		content: '';
		position: absolute;
		width: 100rem;
		height: 1.2rem;
		left: -100%;
	}
	:global(.node-entry:hover:before) {
		background-color: var(--glass-med);
	}
	:global(.dir > *) {
		--spacing: 0.35rem;
		--indent: 0.75rem;
	}
</style>
