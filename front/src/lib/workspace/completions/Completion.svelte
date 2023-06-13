<script lang="ts">
	import type { CompletionEntryMatch } from '$core/completions'
	import { COMPLETION_ICONS } from '$core/icon'
	import { createEventDispatcher } from 'svelte'
	export let expanded = false
	export let completionIndex: number
	export let completion: CompletionEntryMatch
	let elem: HTMLDivElement

	const dispatcher = createEventDispatcher()

	$: if (expanded) {
		elem.scrollIntoView()
	}
</script>

<div
	class="root"
	class:expanded
	bind:this={elem}
	on:click={() => dispatcher('click', {})}
	on:keydown={() => {}}
>
	<div class="insert-text">
		<svelte:component
			this={COMPLETION_ICONS[completionIndex]}
			class="completionIcon"
		/>
		<div class="completion-text">
			{completion.matchParts[0]}
			<span class="highlighted">
				{completion.matchParts[1]}
			</span>
			{completion.matchParts[2]}
		</div>
	</div>
	<span class="description" class:visible={expanded}>
		{#if completion.alias}
			<span>
				{completion.insertText}
			</span>
		{:else}
			{completion.snippetText}
		{/if}
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
	.root:hover {
		background-color: var(--glass-low);
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
	.completion-text {
		display: flex;
		flex-direction: row;
		align-items: center;
	}
</style>
