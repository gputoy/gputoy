<script lang="ts">
	// @ts-nocheck
	import CompletionController from '$core/input'
	import type { Class } from '$gen'
	import { createEventDispatcher, onDestroy, onMount } from 'svelte'

	const dispatch = createEventDispatcher()

	export let key: string
	export let inputClass: Class
	let input: HTMLInputElement
	export let value: any
	let useController = inputClass.ty == 'Str' || inputClass.ty == 'Cmd'

	onMount(() => {
		if (useController) {
			CompletionController.register({
				key,
				class: inputClass,
				elem: input,
				onChange: handleCompletionChange
			})
		}
	})
	onDestroy(() => {
		if (useController) {
			CompletionController.deregister(key)
		}
	})
	function handleBlur(event: Event) {
		if (useController) CompletionController.deattach(key)
	}

	function handleFocus(event: Event) {
		if (useController) CompletionController.attach(key)
	}

	function handleCompletionChange(value: any) {
		dispatch('value', value)
	}

	const ty = inputTy(inputClass.ty)
	$: c = inputClass.c

	function inputTy(ty: string) {
		switch (ty) {
			case 'Int':
			case 'Float':
				return 'number'
			case 'Str':
				return 'text'
		}
	}
	function passChecks(value: any) {
		return true
	}

	function handleChange() {
		console.log('changing? ', { useController })

		if (useController) return
		const maybeVal = input.value.trim()
		console.log('handleChange', maybeVal)
		if (passChecks(maybeVal)) {
			value = maybeVal
		}
	}

	export function focus(preventScroll = false) {
		input?.focus({ preventScroll })
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
/>

<style>
	input {
		width: 50px;
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
</style>
