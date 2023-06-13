<script lang="ts">
	import ControlBar from '../ControlBar.svelte'
</script>

<div class="root">
	<ControlBar />
	<div class="content" />
</div>

<style>
	.root {
		display: flex;
		flex-direction: column;
		height: 100%;
		gap: var(--gap2);
	}
	.content {
		flex: 1 1 auto;
		height: calc(100% - var(--md-nav));
		width: 100%;
		border-radius: var(--pane-radius);
		background-color: black;
	}
</style>
