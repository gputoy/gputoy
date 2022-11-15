<script lang="ts">
	import Icon from '$lib/components/Icon.svelte'
	import { wLayout } from '$stores/project'

	export let title: string
	function handleClick() {
		wLayout.toggleAccordian(title)
	}
	$: open = $wLayout.accordianOpen[title] ?? false
</script>

<div class="accordian-container">
	<div
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
	</div>
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
		background-color: var(--background-content);
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
		padding: 1rem;
		border-bottom: var(--border2);
		min-width: max-content;
	}
	p {
		font-size: var(--sm);
		margin: 0px;
		padding: 4px;
		color: var(--text-accent-color);
	}
</style>
