<script lang="ts">
	import InputController from '$core/input'
	import Completion from './Completion.svelte'

	let completions = InputController.completions()
	let left = 200
	let top = 200
	$: console.log('Completions have changed: ', $completions)
</script>

<div class="completions-container" style={`left: ${left}px; top: ${top}px;`}>
	<div>
		<div>
			{$completions.arg?.name}: {$completions.arg?.ty}
		</div>
		{$completions.arg?.description}
	</div>
	<div class="completion-list">
		{#if $completions.completions.length == 0}
			No completions :(
		{/if}
		{#each $completions.completions as completion, i}
			<Completion {completion} expanded={i == 0} />
		{/each}
	</div>
</div>

<style>
	.completions-container {
		background-color: var(--background-alt);
		z-index: 500;
		height: 150px;
		display: flex;
		flex-direction: column-reverse;
		position: absolute;
		width: 400px;
		border: var(--border);
		padding-block: var(--gap4);
		font-size: var(--sm);
	}
	.completion-list {
		display: flex;

		flex: 1 1 0;
		width: 100%;
		overflow-y: scroll;
		flex-direction: column-reverse;
	}
</style>
