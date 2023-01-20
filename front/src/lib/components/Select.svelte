<script lang="ts">
	import { createEventDispatcher } from 'svelte'
	import IconButton from './buttons/IconButton.svelte'
	import Icon from './Icon.svelte'

	const dispatch = createEventDispatcher()

	export let options: any[]
	export let value: any

	let show = false

	function handleSwitch(value: any) {
		show = false
		dispatch('change', value)
	}

	function handleClick() {
		show = !show
	}
</script>

<div class="container">
	<IconButton size="xs" text={value} on:click={handleClick}>
		<Icon name="chevron-up" stroked thick />
	</IconButton>
	{#if show}
		<ul class="option-list">
			{#each options as option}
				<button
					class:selected={option == value}
					class="list-item"
					on:click={() => handleSwitch(option)}>{option}</button
				>
			{/each}
		</ul>
	{/if}
</div>

<style>
	.container {
		width: fit-content;
		height: fit-content;
		position: relative;
	}
	.option-list {
		position: absolute;
		bottom: 100%;
		left: 0%;
		right: 0%;
		background-color: var(--background-menu);
		margin: 0px;
		border: var(--border2);
		border-bottom: none;
		border-radius: 4px 4px 0px 0px;
		padding-block: 4px;
	}
	.list-item {
		background-color: transparent;
		border: none;
		color: var(--color-primary);
		font-size: var(--xxs);
		width: 100%;
		padding-block: 2px;
		text-align: left;
	}
	.list-item:hover {
		background-color: var(--glass-high);
	}
	.selected {
		background-color: var(--glass-med);
	}
</style>
