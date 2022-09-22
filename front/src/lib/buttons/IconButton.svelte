<script lang="ts">
	export let text: string | undefined = undefined;
	const props = {
		width: '20px',
		height: '20px'
	};
</script>

<button class="icon-button" {...$$restProps} on:click>
	<slot {...props} />
	{#if text}
		<div>
			{text}
		</div>
	{/if}
</button>

<style>
	.icon-button :global(svg) {
		width: var(--icon-sm);
		height: var(--icon-sm);
	}

	div {
		padding: 4px;
	}
	.icon-button {
		font-size: var(--sm);
		display: flex;
		justify-content: center;
		align-items: center;
		height: calc(var(--3xl) - 4px);
		color: var(--text-color);
		background-color: var(--glass-med);
		border: none;
		border-radius: 500px;
	}

	.icon-button:hover {
		color: var(--accent-color);
	}
</style>
