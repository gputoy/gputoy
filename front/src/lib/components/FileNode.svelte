<script lang="ts">
	import {
		getCanonicalName,
		type FileTreeNode,
		type FileTreeNodeChild,
		type FileWithId
	} from '$lib/core/fileTree'

	export let fileNode: FileTreeNode

	function getFileName(f: FileTreeNodeChild) {
		return getCanonicalName(f as FileWithId)
	}
</script>

<li class="dir">
	{fileNode.dir}
	<ul>
		{#each Object.values(fileNode.children) as child}
			{#if 'children' in child}
				<svelte:self fileNode={child} />
			{:else if 'fileName' in child}
				<li class="file">
					{getFileName(child)}
				</li>
			{/if}
		{/each}
	</ul>
</li>

<style>
	.dir {
		font-size: var(--sm);
	}

	ul {
		margin-top: 0.5rem;
		display: flex;
		flex-direction: column;
		padding-left: 1rem;
		list-style: none;
		gap: 0.5rem;
	}

	.file {
		position: relative;
	}
</style>
