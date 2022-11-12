<script lang="ts">
	import { pushAction } from '$lib/core/actions'
	import {
		getCanonicalName,
		type FileTreeNode,
		type FileTreeNodeChild,
		type FileWithId
	} from '$lib/core/fileTree'

	export let fileNode: FileTreeNode
	let open = false

	function getFileName(f: FileTreeNodeChild) {
		return getCanonicalName(f as FileWithId)
	}
	function toggleOpen() {
		open = !open
	}
	function makeFileClickHandler(file: FileWithId) {
		return function () {
			pushAction({
				ty: 'openDocument',
				c: file.id
			})
		}
	}
</script>

{#if fileNode.dir == ''}
	<ul>
		{#each Object.values(fileNode.children) as child}
			{#if 'children' in child}
				<svelte:self fileNode={child} />
			{:else if 'fileName' in child}
				<li class="file entry" on:click={makeFileClickHandler(child)}>
					{getFileName(child)}
				</li>
			{/if}
		{/each}
	</ul>
{:else}
	<li class="dir">
		<span class="entry" on:click={toggleOpen}>
			{fileNode.dir}
		</span>
		{#if open}
			<ul>
				{#each Object.values(fileNode.children) as child}
					{#if 'children' in child}
						<svelte:self fileNode={child} />
					{:else if 'fileName' in child}
						<li class="file entry" on:click={makeFileClickHandler(child)}>
							{getFileName(child)}
						</li>
					{/if}
				{/each}
			</ul>
		{/if}
	</li>
{/if}

<style>
	ul {
		margin-top: 0.5rem;
		display: flex;
		flex-direction: column;
		padding-left: 1rem;
		list-style: none;
		gap: 0.5rem;
		font-size: var(--sm);
	}

	.file {
		position: relative;
	}

	.entry {
		cursor: pointer;
		position: relative;
	}

	.entry::before {
		content: '';
		position: absolute;
		width: 100rem;
		height: 1.2rem;
		left: -100%;
	}

	.entry:hover::before {
		background-color: var(--glass-med);
	}
</style>
