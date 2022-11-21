<script lang="ts">
	export let text: string | undefined = undefined
	export let empty = false
	export let size: string = 'sm'
	type SeriesPositon = 'first' | 'middle' | 'last' | 'none'
	export let series: SeriesPositon = 'none'
	export let smallIcons = false
</script>

<button
	class={size}
	{...$$restProps}
	on:click
	class:left-round={series == 'none' || series == 'first'}
	class:right-round={series == 'none' || series == 'last'}
	class:no-left-border={series == 'middle' || series == 'last'}
	class:small-icons={smallIcons}
	class:empty
>
	<slot />
	{#if text}
		<div>
			{text}
		</div>
	{/if}
</button>

<style>
	div {
		padding: 4px;
		min-width: max-content;
	}
	button {
		display: flex;
		justify-content: center;
		align-items: center;
		background-color: var(--button);
		border: var(--border2);
		color: var(--text-accent-color);
		box-shadow: var(--box-shadow);
		user-select: none;
	}
	button:disabled {
		color: var(--border-secondary);
	}

	button:hover:enabled {
		background-color: var(--button-hover);
		color: var(--text-important);
	}

	button:active:enabled {
		color: var(--text-important);
		background-color: transparent;
		box-shadow: none;
		outline: none;
	}

	.sm {
		padding: 0 4px;
		font-size: var(--xs);
		height: calc(var(--2xl));
		min-width: calc(var(--2xl));
		--border-radius: 4px;
	}

	.sm :global(svg) {
		width: var(--icon-sm);
		height: var(--icon-sm);
	}

	.md {
		font-size: var(--md);
		height: var(--4xl);
		min-width: var(--4xl);
		--border-radius: 6px;
	}

	.md :global(svg) {
		width: var(--icon-md);
		height: var(--icon-md);
	}

	.lg {
		font-size: var(--lg);
		height: var(--5xl);
		min-width: var(--5xl);
	}

	.lg :global(svg) {
		width: var(--icon-lg);
		height: var(--icon-lg);
	}
	.left-round {
		border-top-left-radius: var(--border-radius);
		border-bottom-left-radius: var(--border-radius);
	}
	.right-round {
		border-top-right-radius: var(--border-radius);
		border-bottom-right-radius: var(--border-radius);
	}
	.no-left-border {
		border-left: none;
	}
	.small-icons :global(svg) {
		width: 12px;
		height: 12px;
	}
	.empty {
		border: none;
		background-color: transparent;
		box-shadow: none;
	}
	.empty:hover {
		background-color: transparent !important;
		color: var(--text-important);
	}
	.empty:hover:global(svg) {
		fill: var(--text-important);
	}
</style>
