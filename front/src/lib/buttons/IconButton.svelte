<button class="icon-button" {...$$restProps} on:click>
	<slot />
</button>

<style>
	.icon-button {
		width: var(--3xl);
		height: var(--3xl);
		color: var(--text-color);
		background-color: var(--input-color);
		border: 1px solid var(--border-primary);
		border: none;
	}

	.icon-button:hover {
		color: var(--accent-color);
	}
</style>
