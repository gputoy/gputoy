<script lang="ts">
	import { createEventDispatcher, onMount } from 'svelte'
	import IconButton from './buttons/IconButton.svelte'
	import Icon from './Icon.svelte'
	let input: HTMLInputElement
	export let initValue: string
	let inputValue = initValue
	export let validate: (value: string) => string | undefined
	let validationResult: string | undefined

	$: {
		console.log('validation running', inputValue)
		validationResult = validate(inputValue)
	}
	let dispatch = createEventDispatcher()

	$: {
		console.log('valid', validationResult)
	}

	function handleConfirmEdit(ev: any) {
		dispatch('confirm', { value: inputValue })
		ev.stopPropagation()
	}
	function handleCancelEdit(ev: MouseEvent) {
		dispatch('cancel')
		ev.stopPropagation()
	}
	function handleInputKeyup(ev: KeyboardEvent) {
		if (ev.key == 'Escape') {
			dispatch('cancel')
		}
	}
	onMount(() => {
		let timeout = setTimeout(() => input.focus(), 10)
		return () => clearTimeout(timeout)
	})
</script>

<div class="validated-input">
	<input
		bind:this={input}
		bind:value={inputValue}
		on:submit={handleConfirmEdit}
		on:click|capture|stopPropagation={() => {}}
		on:keyup={handleInputKeyup}
		aria-invalid={!!validationResult}
		class:invalid={!!validationResult}
	/>
	{#if validationResult}
		<div class="validation">
			{validationResult}
		</div>
	{/if}
</div>
<div class="icon-container">
	<IconButton size="xs" empty on:click={handleConfirmEdit}>
		<Icon name="check" stroked size="12px" />
	</IconButton>
	<IconButton size="xs" empty on:click={handleCancelEdit}>
		<Icon name="x" stroked size="12px" />
	</IconButton>
</div>

<style>
	.validated-input {
		position: relative;
	}
	input {
		padding: 2px;
		background-color: transparent;
		z-index: 1;
	}
	.icon-container {
		display: flex;
		flex-direction: row;
		gap: 4px;
	}
	.validation {
		position: absolute;
		right: 4px;
		top: 50%;
		translate: 0 -50%;
		color: var(--color-invalid);
		pointer-events: none;
	}
</style>
