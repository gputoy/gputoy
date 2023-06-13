<script lang="ts">
	import {
		rCompletionIndex,
		rCompletions,
		rCompletionsPosition,
		setCompletionIndex
	} from '$core/completions'
	import { COMPLETION_KEY_TO_INDEX } from '$gen'
	import Completion from './Completion.svelte'

	let completionsHeight: number

	// TODO: move this logic to completions module
	$: completionIndex =
		COMPLETION_KEY_TO_INDEX[$rCompletions.argInfo?.ty ?? 'Empty']
	$: hide = $rCompletions.matches.length == 0
	$: above = ($rCompletionsPosition?.top ?? 0) > 150
	$: x = ($rCompletionsPosition?.x ?? -300) - 19
	$: y = above
		? ($rCompletionsPosition?.top ?? 0) - completionsHeight
		: $rCompletionsPosition?.bottom ?? 0
</script>

<div
	bind:clientHeight={completionsHeight}
	class="completions-container"
	style={`left: ${x}px; top: ${y}px;`}
	class:hide
	class:reverse={!above}
>
	<div class="completion-list">
		{#each $rCompletions.matches as completion, i}
			<Completion
				{completion}
				{completionIndex}
				on:click={() => {
					setCompletionIndex(i)
				}}
				expanded={i == $rCompletionIndex}
			/>
		{/each}
	</div>
</div>

<style>
	.completions-container {
		position: absolute;
		z-index: var(--z-completion);
		border: var(--border);
		font-size: var(--sm);
		width: max-content;
	}
	.completion-list {
		position: relative;
		min-width: 350px;
		max-height: 150px;
		height: fit-content;
		background-color: var(--background-alt);
		display: flex;
		flex: 1 1 0;
		width: 100%;
		overflow-y: scroll;
		flex-direction: column-reverse;
	}
	.reverse {
		flex-direction: column;
	}
	.hide {
		visibility: hidden;
	}
</style>
