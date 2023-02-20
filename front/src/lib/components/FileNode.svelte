<script lang="ts">
	import { pushAction } from '$core/actions'
	import {
		getCanonicalName,
		type FileTreeNode,
		type FileTreeNodeChild,
		type FileWithId
	} from '$core/files'
	import FileIcon from '$lib/components/FileIcon.svelte'
	import Icon from '$lib/components/Icon.svelte'
	import { dActiveFile, wLayout } from '$stores'

	export let fileNode: FileTreeNode
	$: open = $wLayout.fileTreeState[fileNode.absoluteDir]?.open ?? false

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

<!--  -->
{#if fileNode.dir == ''}
	<ul>
		{#each Object.values(fileNode.children) as child}
			{#if 'children' in child}
				<svelte:self fileNode={child} />
			{:else if 'fileName' in child}
				<button
					class="file entry"
					on:click={makeFileClickHandler(child)}
					class:active={child.id == $dActiveFile}
				>
					<FileIcon
						extension={getFileExtension(child)}
						size={14}
						class="file-icon"
					/>
					{getFileName(child)}
				</button>
			{/if}
		{/each}
	</ul>
{:else}
	<li class="dir">
		<button class="entry" on:click={toggleOpen}>
			<Icon
				name="chevron-right"
				size="16px"
				rotation={open ? '90deg' : '0deg'}
			/>
			{fileNode.dir}
		</button>
		{#if open}
			<ul>
				{#each Object.values(fileNode.children) as child}
					{#if 'children' in child}
						<svelte:self fileNode={child} />
					{:else if 'fileName' in child}
						<button
							class="file entry"
							on:click={makeFileClickHandler(child)}
							class:active={child.id == $dActiveFile}
						>
							<FileIcon
								extension={getFileExtension(child)}
								size={14}
								class="file-icon"
							/>
							{getFileName(child)}
						</button>
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
		user-select: none;
	}

	.entry::before {
		content: '';
		position: absolute;
		width: 100rem;
		height: 1.2rem;
		left: -100%;
	}

	.entry:hover::before {
		background-color: var(--glass-low);
	}

	button {
		font-size: var(--xs);
		background: transparent;
		color: var(--text-color);
		border: none;
	}
	.active::before {
		background-color: var(--glass-med);
	}
</style>
