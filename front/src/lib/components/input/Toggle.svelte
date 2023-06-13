<script lang="ts">
	export let value: any

	function handleClick() {
		value = !value
	}
	function handleKeypress() {}
</script>

<div
	class={`${$$restProps.class || 'sm'} root`}
	on:click={handleClick}
	on:keypress={handleKeypress}
>
	<div class="val">
		{#if value}
			true
		{:else}
			false
		{/if}
	</div>
	<div class="switch" class:toggled={value}>
		<div class="knob" class:toggled={value} />
	</div>
</div>

<style>
	.root {
		display: flex;
		border-radius: var(--pane-radius);
		background-color: var(--input-color);
		width: 100%;
		align-items: center;
		justify-content: space-between;
		cursor: pointer;
		user-select: none;
		padding-inline: var(--gap4);
		gap: var(--gap4);
		font-weight: normal;
	}
	.root:hover {
		background-color: var(--input-hover);
	}
	.val {
		align-items: center;
		width: 100%;
	}
	.switch {
		display: flex;
		background-color: var(--button-color);
		border-radius: 500px;
		height: calc(100% - var(--gap8));
		align-items: center;
		justify-content: left;
		aspect-ratio: 2;
		transition: justify-content var(--ui-speed) ease;
	}
	.switch.toggled {
		justify-content: right;
	}
	.knob {
		margin-inline: var(--gap4);
		height: calc(100% - var(--gap8));
		background-color: var(--input-color);
		border-radius: 500px;
		aspect-ratio: 1;
	}
	.knob.toggled {
		background-color: var(--accent-color);
	}
</style>
