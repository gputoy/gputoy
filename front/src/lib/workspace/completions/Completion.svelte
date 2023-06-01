<script lang="ts">
	import { ICONS } from '$core/completions'
	import type { CompletionEntry } from '$gen'
	export let expanded = false
	export let completionIndex: number
	export let completion: CompletionEntry
	let elem: HTMLDivElement
</script>

<div class="root" class:expanded bind:this={elem}>
	<div class="insert-text">
		<svelte:component this={ICONS[completionIndex]} class="completionIcon" />
		{completion.insertText}
	</div>
	<span class="description" class:visible={expanded}>
		{completion.name}
	</span>
</div>

<style>
	.root {
		display: flex;
		justify-content: space-between;
		padding-inline: var(--gap4);
		padding-block: 1px;
		gap: var(--gap8);
		justify-content: space-between;
		align-items: center;
		min-width: max-content;
		font-size: var(--sm);
	}
	.expanded {
		background-color: var(--glass-med);
		/* font-weight: bold; */
	}
	.insert-text {
		font-family: var(--font-mono);
		display: flex;
		align-items: center;
		color: var(--text-color);
		gap: var(--gap4);
	}

	.description {
		color: var(--text-low);
		font-weight: bold;
		font-size: smaller;
		visibility: hidden;
	}
	.visible {
		visibility: visible;
	}
</style>
