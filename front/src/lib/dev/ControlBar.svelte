<script lang="ts">
	import IconButton from '$lib/buttons/IconButton.svelte'
	import { build, render as context_render } from '$lib/context'
	import { clearProject, getProject } from '$stores/project'
	import Icon from 'svelte-awesome'
	import play from 'svelte-awesome/icons/play'

	let x = context_render

	function handleBuild() {
		build(getProject())
	}
	function handleClear() {
		clearProject()
	}
</script>

<div id="controlbar-container">
	<div class="left" />
	<div class="middle">
		<IconButton size="md">
			<Icon data={play} />
		</IconButton>

		<IconButton on:click={handleBuild}>Build</IconButton>

		<IconButton on:click={context_render}>Render</IconButton>
		<IconButton on:click={handleClear}>Clear</IconButton>
	</div>

	<div class="right" />
</div>

<style>
	#controlbar-container {
		width: 100%;
		border-bottom: var(--border-secondary) solid 1px;
		height: 48px;
		background-color: var(--tertiary-color);
		display: flex;
		flex-direction: row;
		justify-content: space-between;
	}

	.left {
		flex-grow: 1;
	}

	.middle {
		flex-grow: 0;
		display: flex;
		justify-content: center;
		align-items: center;
	}

	.right {
		flex-grow: 1;
	}
</style>
