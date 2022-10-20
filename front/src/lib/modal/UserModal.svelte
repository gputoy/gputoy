<script lang="ts">
	import Login from '$lib/user/Login.svelte'
	import UserDashboard from '$lib/user/UserDashboard.svelte'
	import { fade } from 'svelte/transition'
	import { user } from '../../stores/auth'
	export let show: boolean = false
	export let onHide: (() => void) | null | undefined
	let ref: HTMLElement | undefined

	$: {
		if (show) document?.getElementById('login-focus')?.focus()
	}

	const _fade = {
		duration: 50
	}

	function onHandleBlur(event: any) {
		console.log('blur ran for ', event)
		if (!event) return
		if (!event.currentTarget.contains(event.relatedTarget) && onHide) {
			onHide()
		}
	}
</script>

<dialog
	class="modal-frame"
	in:fade={_fade}
	out:fade={_fade}
	bind:this={ref}
	open={show}
	on:blur|capture={onHandleBlur}
>
	{#if $user}
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
