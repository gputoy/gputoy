<script lang="ts">
	import { pushAction } from '$lib/core/actions'
	import {
		getCanonicalName,
		type FileTreeNode,
		type FileTreeNodeChild,
		type FileWithId
	} from '$lib/core/fileTree'
	import { wLayout } from '$stores/project'
	import Icon from 'svelte-awesome'
	import angleRight from 'svelte-awesome/icons/angleRight'
	import FileIcon from './FileIcon.svelte'

	export let fileNode: FileTreeNode
	let open = false

	function getFileName(f: FileTreeNodeChild) {
		return getCanonicalName(f as FileWithId)
	}
	function getFileExtension(f: FileTreeNodeChild) {
		return (f as FileWithId).extension
	}
	function toggleOpen() {
		wLayout.toggleDirOpen(fileNode.absoluteDir)
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
					<FileIcon extension={getFileExtension(child)} size={14} />
					{getFileName(child)}
				</li>
			{/if}
		{/each}
	</ul>
{:else}
	<li class="dir">
		<span class="entry" on:click={toggleOpen}>
			<Icon
				data={angleRight}
				class="dir-icon"
				style="width: 16px;{open ? 'transform: rotate(90deg);' : ''}"
			/>
			{fileNode.dir}
		</span>
		{#if $wLayout.fileTreeState[fileNode.absoluteDir]?.open ?? false}
			<ul>
				{#each Object.values(fileNode.children) as child}
					{#if 'children' in child}
						<svelte:self fileNode={child} />
					{:else if 'fileName' in child}
						<li class="file entry" on:click={makeFileClickHandler(child)}>
							<FileIcon extension={getFileExtension(child)} size={14} class="file-icon" />
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
		font-size: var(--xs);
	}

	.file {
		position: relative;
	}

	.entry {
		display: flex;
		align-items: center;
		cursor: pointer;
		position: relative;
		gap: 4px;
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
	.dir-icon {
		width: 16px;
	}
	.file-icon {
		position: absolute;
	}
</style>
