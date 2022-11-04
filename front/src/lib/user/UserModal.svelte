<script lang="ts">
	import Login from '$lib/user/Login.svelte'
	import UserDashboard from '$lib/user/UserDashboard.svelte'
	import { wUser } from '$stores/auth'
	import { wUserModalOpen } from '$stores/ui'
	let dialog: HTMLDialogElement

	$: {
		if (dialog) {
			if ($wUserModalOpen) dialog.show()
			else dialog.close()
		}
	}
</script>

<dialog class="modal-frame" bind:this={dialog}>
	{#if $wUser}
		<UserDashboard />
	{:else}
		<Login />
	{/if}
</dialog>

<style>
	.modal-frame {
		z-index: 10;
		position: absolute;
		top: 50%;
		left: 50%;
		translate: -50% -50%;
		height: fit-content;
		width: fit-content;
		border: none;
		background-color: transparent;
	}
</style>
