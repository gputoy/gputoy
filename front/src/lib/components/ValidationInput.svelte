<script lang="ts">
	import { createEventDispatcher, onMount } from 'svelte'
	import IconButton from './buttons/IconButton.svelte'
	import Icon from './Icon.svelte'
	let input: HTMLInputElement
	export let initValue: string
	let inputValue = initValue
	export let validate: (value: string) => string | undefined
	let validationErrors: string | undefined

	$: {
		validationErrors = validate(inputValue)
	}
	let dispatch = createEventDispatcher()

	function handleConfirmEdit(ev: any) {
		if (!validationErrors) dispatch('confirm', { value: inputValue })
		else handleCancelEdit(ev)
		ev.stopPropagation()
	}
	function handleCancelEdit(ev: MouseEvent) {
		dispatch('cancel')
		ev.stopPropagation()
	}
	function handleInputKeyup(ev: KeyboardEvent) {
		if (ev.key == 'Escape') {
			dispatch('cancel')
		} else if (ev.key == 'Enter' && !validationErrors) {
			dispatch('confirm', { value: inputValue })
		}
	}
	onMount(() => {
		let timeout = setTimeout(() => input.focus(), 10)
		return () => clearTimeout(timeout)
	})
</script>

<div class="validated-input">
	<input
		type="text"
		bind:this={input}
		bind:value={inputValue}
		on:submit={handleConfirmEdit}
		on:click|capture|stopPropagation={() => {}}
		on:keyup={handleInputKeyup}
		aria-invalid={!!validationErrors}
		class:invalid={!!validationErrors}
		spellcheck={false}
	/>
	{#if validationErrors}
		<div class="validation">
			{validationErrors}
		</div>
	{/if}
</div>
<div class="icon-container">
	<IconButton
		size="xs"
		empty
		on:click={handleConfirmEdit}
		disabled={!!validationErrors}
	>
		<Icon name="check" stroked size="12px" />
	</IconButton>
	<IconButton size="xs" empty on:click={handleCancelEdit}>
		<Icon name="x" stroked size="12px" />
	</IconButton>
</div>

<style>
	.validated-input {
		flex: 1 1 auto;
		position: relative;
	}
	input {
		padding: 1px;
		background-color: transparent;
		z-index: 1;
		width: 0;
		min-width: 100%;
		margin-right: 2px;
		box-sizing: content-box;
		margin-left: -1px;
	}
	.icon-container {
		flex: 0 0 auto;
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
