<script lang="ts">
	export let text: string | undefined = undefined;
	export let size: string = 'sm';
</script>

<button class={'icon-button ' + size} {...$$restProps} on:click>
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
	}
	.icon-button {
		display: flex;
		justify-content: center;
		align-items: center;
		background-color: var(--button);
		outline: 1px solid var(--border-secondary);
		border: none;
		border-radius: 500px;
		color: var(--text-color);
		transition: all 0.15s ease;
	}

	.sm {
		font-size: var(--sm);
		height: calc(var(--3xl) - 4px);
		min-width: calc(var(--3xl) - 4px);
	}

	.sm :global(svg) {
		width: var(--icon-sm);
		height: var(--icon-sm);
	}

	.md {
		font-size: var(--md);
		height: var(--4xl);
		min-width: var(--4xl);
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

	.icon-button:hover {
		background-color: var(--button-hover);
	}

	.icon-button:active {
		color: var(--accent-color);
		background-color: var(--button-active);
		outline: none;
	}
</style>
