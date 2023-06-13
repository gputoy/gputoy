<script lang="ts">
	import { rAccordianOpen, toggleAccordian } from '$core/layout'
	import Icon from '$lib/components/Icon.svelte'
	import { slide } from 'svelte/transition'

	export let title: string
	function handleClick() {
		toggleAccordian(title)
	}
	$: open = $rAccordianOpen[title] ?? false
</script>

<div class="accordian-container">
	<button class="accordian-title" on:click={handleClick}>
		<div class="accordian-title-left">
			<Icon
				class="xs"
				stroked
				name="chevron-right"
				rotation={open ? '90deg' : '0deg'}
			/>
			{title}
		</div>

		<div class="accordian-title-right">
			<slot name="menu" />
		</div>
	</button>
	<div
		class="accordian-content hide"
		transition:slide={{ duration: 50 }}
		class:unhide={open}
	>
		<slot name="content" />
	</div>
</div>

<style>
	.accordian-container {
		display: flex;
		height: fit-content;
		width: 100%;
		flex-direction: column;
		background-color: var(--background-alt);
		border-radius: var(--pane-radius);
	}
	.accordian-title {
		height: var(--sm-nav);
		font-size: var(--sm);
		border-radius: var(--pane-radius);
		padding-inline: var(--gap4);
		display: flex;
		align-items: center;
		justify-content: space-between;
		flex: 0 0 auto;
		user-select: none;
		background: transparent;
		font-weight: bold;
		color: var(--text-accent-color);
	}
	.accordian-title:hover {
		background-color: var(--background-nav);
	}
	.accordian-title-left {
		display: flex;
		align-items: center;
		gap: var(--nav-gap);
	}
	.accordian-title-right {
		display: flex;
		align-items: center;
	}
	.accordian-content {
		flex: 1 1 auto;
		padding: 0;
		/* border-bottom: var(--border); */
		min-width: 100px;
	}
</style>
