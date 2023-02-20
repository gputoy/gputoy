<script lang="ts">
	import { pushAction } from '$core/actions'
	import {
		getCanonicalName,
		type FileTreeNodeChild,
		type FileWithId
	} from '$core/files'
	import IconButton from '$lib/components/buttons/IconButton.svelte'
	import Icon from '$lib/components/Icon.svelte'
	import { dActiveFile, wLayout } from '$stores'
	import FileIcon from './FileIcon.svelte'

	const FILE_ICON_SIZE_PX = 12
	export let node: FileTreeNodeChild
	export let open = false
	export let clazz = ''

	function getFileName(f: FileTreeNodeChild) {
		return getCanonicalName(f as FileWithId)
	}
	function getFileExtension(f: FileTreeNodeChild) {
		return (f as FileWithId).extension
	}
	function makeFileClickHandler(node: FileTreeNodeChild) {
		if ('id' in node) {
			// is file, return handler that opens document
			return () => {
				pushAction({
					ty: 'openDocument',
					c: node.id
				})
			}
		} else {
			// is directory, toggle directory open
			return () => {
				wLayout.toggleDirOpen(node.absoluteDir)
			}
		}
	}
	function handleEdit() {
		console.log('handling edit')
	}
	function handleDelete() {
		console.log('handling edit')
	}
</script>

<button
	class={clazz}
	on:click={makeFileClickHandler(node)}
	class:active={'id' in node && node.id == $dActiveFile}
>
	<div class="content left">
		{#if 'id' in node}
			<FileIcon
				extension={getFileExtension(node)}
				size={FILE_ICON_SIZE_PX}
				class="file-icon"
			/>
			<p class="title">
				{getFileName(node)}
			</p>
		{:else}
			<Icon name="chevron-right" stroked rotation={open ? '90deg' : '0deg'} />
			<p class="title">
				{node.dir}
			</p>
		{/if}
	</div>
	<div class="content right">
		<IconButton size="xs" empty on:click={handleEdit}>
			<Icon name="edit-2" stroked size="12px" />
		</IconButton>
		<IconButton size="xs" empty on:click={handleDelete}>
			<Icon name="trash" stroked size="12px" />
		</IconButton>
	</div>
</button>

<style>
	.title {
		margin: 0px;
		color: var(--text-color);
		min-width: 0;
		text-overflow: ellipsis;
		width: min-content;
	}
	.left {
		flex: 1 1 auto;
	}
	.right {
		flex: 0 0 auto;
		min-width: max-content;
		visibility: hidden;
	}
	button:hover > .right {
		visibility: visible;
	}
	.active p {
		color: var(--text-important);
	}
	.active::before {
		background-color: var(--glass-high);
	}
	.content {
		display: flex;
		flex-direction: row;
		gap: 4px;
		align-items: center;
	}
</style>
