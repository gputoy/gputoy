<script lang="ts">
	import {
		getCanonicalName,
		pathParent,
		validateRename,
		type FileTreeNode,
		type FileTreeNodeChild,
		type FileWithId
	} from '$core/files'

	import { pushAction } from '$core/actions'
	import { dActiveFile, toggleDirOpen } from '$core/layout'
	import Icon from '$lib/components/Icon.svelte'
	import { wUserDeleting, wUserRenaming } from '$stores'
	import { slide } from 'svelte/transition'
	import ValidationInput from '../ValidationInput.svelte'
	import FileIcon from './FileIcon.svelte'

	const FILE_ICON_SIZE_PX = 12
	export let node: FileTreeNodeChild
	export let open = false
	export let clazz = ''

	$: isFile = 'id' in node
	$: identifier = isFile
		? (node as FileWithId).id
		: (node as FileTreeNode).absoluteDir
	$: name = isFile
		? getCanonicalName(node as FileWithId)
		: (node as FileTreeNode).dir
	$: isEditing = $wUserRenaming == identifier
	$: isDeleting = $wUserDeleting == identifier
	let validationResult: string | undefined

	$: renameValue = name
	$: {
		if (isEditing) validationResult = validateRename(node, renameValue)
	}
	function getFileExtension(f: FileTreeNodeChild) {
		return (f as FileWithId).extension
	}
	function makeClickHandler(node: FileTreeNodeChild) {
		if ('id' in node) {
			// is file, return handler that opens document
			return () => {
				pushAction({
					ty: 'openTab',
					c: node.id
				})
			}
		} else {
			// is directory, toggle directory open
			return () => {
				toggleDirOpen(node.absoluteDir)
			}
		}
	}
	function handleEdit(ev: MouseEvent) {
		wUserDeleting.set(null)
		wUserRenaming.set(identifier)
		ev.stopPropagation()
	}
	function handleConfirmEdit(ev: CustomEvent<any>) {
		let newPath = pathParent(identifier) + '/' + ev.detail.value
		pushAction({
			ty: 'move',
			c: { src: identifier, dest: newPath, isDir: !isFile }
		})
		wUserRenaming.set(null)
	}
	function handleCancelEdit() {
		wUserRenaming.set(null)
	}

	function handleDelete(ev: MouseEvent) {
		wUserRenaming.set(null)
		wUserDeleting.set(identifier)
		ev.stopPropagation()
	}
	function handleConfirmDelete(ev: MouseEvent) {
		pushAction({ ty: 'delete', c: { path: identifier, isDir: !isFile } })
		wUserDeleting.set(null)
		ev.stopPropagation()
	}
	function handleCancelDelete(ev: MouseEvent) {
		wUserDeleting.set(null)
		ev.stopPropagation()
	}
</script>

<button
	class={clazz}
	class:root={true}
	on:click={makeClickHandler(node)}
	class:active={'id' in node && node.id == $dActiveFile}
	transition:slide={{ duration: 50 }}
>
	<!-- Icon, either a file icon or directory chevron -->
	{#if isFile}
		<FileIcon
			extension={getFileExtension(node)}
			size={FILE_ICON_SIZE_PX}
			class="file-icon"
		/>
	{:else}
		<Icon name="chevron-right" stroked rotation={open ? '90deg' : '0deg'} />
	{/if}
	<div class="content" class:strikethrough={isDeleting}>
		{#if isEditing}
			<ValidationInput
				initValue={name}
				validate={(val) => validateRename(node, val)}
				on:confirm={handleConfirmEdit}
				on:cancel={handleCancelEdit}
			/>
		{:else if isDeleting}
			{name}
			<div class="icon-container">
				<button class="xs right-flat emtpy" on:click={handleConfirmDelete}>
					<Icon name="check" stroked size="12px" />
				</button>
				<button class="xs left-flat empty" on:click={handleCancelDelete}>
					<Icon name="x" stroked size="12px" />
				</button>
			</div>
		{:else}
			{name}
			<div class="icon-container hidden">
				<button class="xs right-flat emtpy" on:click={handleEdit}>
					<Icon name="edit-2" stroked />
				</button>
				<button class="xs left-flat empty" on:click={handleDelete}>
					<Icon name="trash" stroked />
				</button>
			</div>
		{/if}
	</div>
</button>

<style>
	.root:hover {
		background-color: transparent;
	}
	.active::before {
		background-color: var(--glass-high);
	}
	.content {
		display: flex;
		flex-direction: row;
		gap: 4px;
		align-items: center;
		justify-content: space-between;
		flex: 1 1 auto;
		max-width: 100%;
		white-space: nowrap;
		text-overflow: ellipsis;
		font-size: var(--sm);
	}
	.icon-container {
		display: flex;
		flex-direction: row;
	}
	.hidden {
		visibility: hidden;
	}
	button:hover .hidden {
		visibility: visible;
	}
	.strikethrough {
		text-decoration: line-through;
		color: var(--color-invalid);
	}
</style>
