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
	import IconButton from '$lib/components/buttons/IconButton.svelte'
	import Icon from '$lib/components/Icon.svelte'
	import { wUserDeleting, wUserRenaming } from '$stores'
	import { slide } from 'svelte/transition'
	import ValidationInput from '../ValidationInput.svelte'
	import FileIcon from './FileIcon.svelte'

	const FILE_ICON_SIZE_PX = 12
	export let node: FileTreeNodeChild
	export let open = false
	export let clazz = ''

	let isFile = 'id' in node
	let identifier = isFile
		? (node as FileWithId).id
		: (node as FileTreeNode).absoluteDir
	let name = isFile
		? getCanonicalName(node as FileWithId)
		: (node as FileTreeNode).dir
	$: isEditing = $wUserRenaming == identifier
	let validationResult: string | undefined

	$: renameValue = name
	$: {
		validationResult = validateRename(node, renameValue)
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
				toggleDirOpen(node.absoluteDir)
			}
		}
	}
	function handleEdit(ev: MouseEvent) {
		wUserDeleting.set(null)
		wUserRenaming.set(identifier)
		ev.stopPropagation()
	}
	function handleDelete(ev: MouseEvent) {
		wUserRenaming.set(null)
		wUserDeleting.set(identifier)
		ev.stopPropagation()
	}
	function handleConfirmEdit(ev: CustomEvent<any>) {
		let newPath = pathParent(identifier) + '/' + ev.detail.value
		pushAction({ ty: 'move', c: [identifier, newPath] })
		wUserRenaming.set(null)
	}
	function handleCancelEdit() {
		wUserRenaming.set(null)
	}
</script>

<button
	class={clazz}
	on:click={makeFileClickHandler(node)}
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
	<div class="content">
		{#if isEditing}
			<ValidationInput
				initValue={name}
				validate={(val) => validateRename(node, val)}
				on:confirm={handleConfirmEdit}
				on:cancel={handleCancelEdit}
			/>
		{:else}
			{name}
			<div class="icon-container hidden">
				<IconButton size="xs" empty on:click={handleEdit}>
					<Icon name="edit-2" stroked size="12px" />
				</IconButton>
				<IconButton size="xs" empty on:click={handleDelete}>
					<Icon name="trash" stroked size="12px" />
				</IconButton>
			</div>
		{/if}
	</div>
</button>

<style>
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
		text-overflow: ellipsis;
	}
	.icon-container {
		display: flex;
		flex-direction: row;
		gap: 4px;
	}
	.hidden {
		visibility: hidden;
	}
	button:hover .hidden {
		visibility: visible;
	}
</style>
