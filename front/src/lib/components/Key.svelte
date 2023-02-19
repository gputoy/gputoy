<script lang="ts">
	export let keycode: string | null
</script>

<span>{keycode ?? ''}</span>

<style>
	span {
		display: inline;
		white-space: nowrap;
		min-width: max-content;
		background-color: var(--glass-med);
		padding: 2px;
		border-radius: 3px;
		height: fit-content;
		user-select: none;
	}
</style>
