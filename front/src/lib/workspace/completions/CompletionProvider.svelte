<script lang="ts">
	import InputController from '$core/input'
	import { COMPLETION_KEY_TO_INDEX } from '$gen'
	import Completion from './Completion.svelte'

	let completions = InputController.completions()
	let completionsIndex = InputController.completionsIndex()
	let location = InputController.completionsLocation()
	let completionsHeight: number
	let left = 200
	let top = 200

	$: completionIndex = COMPLETION_KEY_TO_INDEX[$completions.arg?.ty ?? 'Empty']
	$: hide = $completions.completions.length == 0
	$: console.log($location)
	$: x = ($location?.x ?? -300) - 19
	$: y = ($location?.y ?? 0) - completionsHeight
</script>

<div
	bind:clientHeight={completionsHeight}
	class="completions-container"
	style={`left: ${x}px; top: ${y}px;`}
	class:hide
>
	<!-- <div>
		<div>
			{$completions.arg?.name}: {$completions.arg?.ty}
		</div>
		{$completions.arg?.description}
	</div> -->
	<div class="completion-list">
		{#each $completions.completions as completion, i}
			<Completion
				{completion}
				{completionIndex}
				expanded={i == $completionsIndex}
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
	.hide {
		visibility: hidden;
	}
</style>
