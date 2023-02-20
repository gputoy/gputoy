<script lang="ts">
	import Icon from '$lib/components/Icon.svelte'
	import { wLayout } from '$stores'

	export let title: string
	function handleClick() {
		wLayout.toggleAccordian(title)
	}
	$: open = $wLayout.accordianOpen[title] ?? false
</script>

<div class="accordian-container">
	<button
		class="accordian-title"
		on:click={handleClick}
		style="border-bottom: {open ? 'none' : 'var(--border2)'};"
	>
		<div class="accordian-title-left">
			<Icon stroked name="chevron-right" rotation={open ? '90deg' : '0deg'} />
			<p>
				{title}
			</p>
		</div>

		<div class="accordian-title-right">
			<slot name="menu" />
		</div>
	</button>
	{#if open}
		<div class="accordian-content">
			<slot name="content" />
		</div>
	{/if}
</div>

<style>
	.accordian-container {
		display: flex;
		height: fit-content;
		flex-direction: column;
	}
	.accordian-title {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding-inline: 0.25rem;
		gap: 0.5rem;
		flex: 0 0 auto;
		user-select: none;
	}
	.accordian-title:hover {
		background-color: var(--glass-med);
	}
	.accordian-title-left {
		display: flex;
		align-items: center;
	}
	.accordian-title-right {
		display: flex;
		align-items: center;
	}
	.accordian-content {
		flex: 1 1 auto;
		padding: 0;
		border-bottom: var(--border2);
		min-width: max-content;
	}
	p {
		font-size: var(--sm);
		margin: 0px;
		padding: 4px;
		font-weight: bold;
		color: var(--text-accent-color);
	}
	button {
		font-size: var(--xs);
		background: transparent;
		color: var(--text-color);
		border: none;
	}
</style>
