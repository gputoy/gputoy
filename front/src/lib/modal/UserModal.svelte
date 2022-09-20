<script lang="ts">
	import Login from '$lib/forms/Login.svelte';
	import { fade } from 'svelte/transition';
	import { user } from '../../stores/auth';
	export let show: boolean = false;
	export let onHide: (() => void) | null | undefined;
	let ref: HTMLElement | undefined;

	$: {
		if (show) document?.getElementById('login-focus')?.focus();
	}

	const _fade = {
		duration: 50
	};

	function onHandleBlur(event) {
		if (!event) return;
		if (!event.currentTarget.contains(event.relatedTarget) && onHide) {
			onHide();
		}
	}
</script>

{#if show}
	<div
		class="modal-frame"
		on:blur|preventDefault|capture={onHandleBlur}
		in:fade={_fade}
		out:fade={_fade}
		bind:this={ref}
	>
		{#if $user}
			<div>
				{JSON.stringify($user, null, 4)}
			</div>
		{:else}
			<Login />
		{/if}
	</div>
{/if}

<style>
	.modal-frame {
		position: absolute;
		top: 50%;
		left: 50%;
		translate: -50% -50%;
		height: fit-content;
		width: fit-content;
	}
</style>
