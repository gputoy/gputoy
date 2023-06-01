<script lang="ts">
	// @ts-nocheck
	import InputController from '$core/input'
	import type { ConfigValueClass } from '$gen'
	import { createEventDispatcher, onDestroy, onMount } from 'svelte'

	const dispatch = createEventDispatcher()

	export let key: string
	export let inputClass: ConfigValueClass
	let input: HTMLInputElement
	export let value: any
	let useController = inputClass.ty == 'StrClass' || inputClass.ty == 'CmdClass'

	onMount(() => {
		if (useController) {
			InputController.register({
				key,
				class: inputClass,
				elem: input,
				onChange: handleCompletionChange
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

	function handleCompletionChange(value: any) {
		dispatch('value', value)
	}

	const ty = inputTy(inputClass.ty)
	$: c = inputClass.c

	function inputTy(ty: string) {
		switch (ty) {
			case 'IntClass':
			case 'FloatClass':
				return 'number'
			case 'StrClass':
				return 'text'
		}
	}
	function passChecks(value: any) {
		return true
	}

	function handleChange() {
		if (useController) return
		const maybeVal = input.value.trim()
		if (passChecks(maybeVal)) {
			value = maybeVal
		}
	}

	export function focus(preventScroll = false) {
		input?.focus({ preventScroll })
	}

	export function clear() {
		if (input) input.value = ''
	}
</script>

<input
	bind:this={input}
	{value}
	type={ty}
	class={`${$$restProps.class || 'sm'} ${inputClass.ty}`}
	min={c.min}
	max={c.max}
	step={c.step}
	pattern={c.regex}
	on:input={handleChange}
	on:focus={handleFocus}
	on:blur={handleBlur}
	class:cmd={inputClass.ty == 'CmdClass'}
/>

<style>
	input {
	}
	input[type='number'] {
		--webkit-appearance: textfield;
		--moz-appearance: textfield;
		appearance: textfield;
	}
	input[type='number']::-webkit-inner-spin-button,
	input[type='number']::-webkit-outer-spin-button {
		--webkit-appearance: none;
	}
	.cmd {
		background-color: transparent;
		font-family: var(--font-mono);
		font-size: var(--sm);
	}
</style>
