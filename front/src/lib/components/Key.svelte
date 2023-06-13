<script lang="ts">
	export let keycode: string | null
</script>

<span>{keycode ?? ''}</span>

<style>
	span {
		display: inline;
		white-space: nowrap;
		min-width: max-content;
		background-color: var(--input-color);
		padding: 2px;
		border-radius: 3px;
		height: fit-content;
		user-select: none;
		font-weight: bold;
		font-family: var(--font-mono);
	}
</style>
