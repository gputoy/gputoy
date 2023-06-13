<script lang="ts">
	// @ts-nocheck
	import InputController from '$core/input'
	import type { CompletionKey, ConfigClass } from '$gen'
	import { createEventDispatcher, onDestroy, onMount } from 'svelte'

	const dispatch = createEventDispatcher()

	export let key: string
	export let inputClass: ConfigClass | null
	export let completionKey: CompletionKey | null
	let input: HTMLInputElement
	export let value: any
	$: useController = !inputClass || inputClass?.ty == 'StrClass'

	$: ty = inputClass ? inputTy(inputClass.ty) : 'text'
	$: c = inputClass?.c

	function inputTy(ty: string) {
		switch (ty) {
			case 'IntClass':
			case 'FloatClass':
				return 'number'
			case 'StrClass':
				return 'text'
		}
	}

	onMount(() => {
		if (useController) {
			InputController.register({
				key,
				elem: input,
				descriptor: completionKey
					? {
							completionKey
					  }
					: undefined,
				onSubmit: handleSubmit,
				onChange: handleChange
			})
		}
	})
	onDestroy(() => {
		if (useController) {
			InputController.deregister(key)
		}
	})
	function handleFocus(_event: Event) {
		if (useController) InputController.attach(key)
	}
	function handleBlur(_event: Event) {
		if (useController) InputController.deattach()
	}

	function handleSubmit(value: any) {
		dispatch('submit', value)
	}
	function handleChange(value: any) {
		dispatch('change', value)
	}

	export function focus(preventScroll = false) {
		input.focus()
	}

	export function clear() {
		InputController.get(key)?.clear()
	}
</script>

<input
	bind:this={input}
	{value}
	type={ty}
	class={`${$$restProps.class || 'sm'} ${inputClass?.ty ?? ''}`}
	min={c?.min}
	max={c?.max}
	step={c?.step}
	pattern={c?.regex}
	on:focus={handleFocus}
	on:blur={handleBlur}
	class:cmd={inputClass == null}
	spellcheck={false}
	{...$$restProps}
/>

<style>
	input[type='number'] {
		--webkit-appearance: textfield;
		--moz-appearance: textfield;
		appearance: textfield;
	}
	input[type='number']::-webkit-inner-spin-button,
	input[type='number']::-webkit-outer-spin-button {
		--webkit-appearance: none;
	}
	input {
		width: 100%;
	}
	.cmd {
		background-color: transparent;
		font-family: var(--font-mono);
		font-size: var(--sm);
	}
</style>
