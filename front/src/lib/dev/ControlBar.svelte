<script lang="ts">
	import IconButton from '$lib/components/buttons/IconButton.svelte'
	import { build, render as context_render } from '$lib/core/context'
	import { clearProject, getProject } from '$stores/project'

	import Icon from 'svelte-awesome'
	import circle from 'svelte-awesome/icons/circle'
	import pause from 'svelte-awesome/icons/pause'
	import play from 'svelte-awesome/icons/play'
	import square from 'svelte-awesome/icons/square'

	let x = context_render
	let playing = false

	function handleBuild() {
		build(getProject())
	}
	function handleClear() {
		clearProject()
	}
</script>

<div id="controlbar-container">
	<div class="left button-container">
		<IconButton smallIcons>
			<Icon data={square} />
		</IconButton>
	</div>
	<div class="middle button-container">
		<IconButton series="first" smallIcons>
			{#if playing}
				<Icon data={pause} />
			{:else}
				<Icon data={play} />
			{/if}
		</IconButton>
		<IconButton series="middle" smallIcons>
			<Icon data={square} />
		</IconButton>
		<div class="info">0.0 fps</div>
		<IconButton series="last" smallIcons>
			<Icon data={circle} />
		</IconButton>
	</div>

	<div class="right button-container">
		<IconButton on:click={context_render} series="first">Render</IconButton>
		<IconButton on:click={handleClear} series="last">Clear</IconButton>
	</div>
</div>

<style>
	#controlbar-container {
		width: calc(100% - 8px);
		border-bottom: var(--border-secondary) solid 1px;
		height: 32px;
		background-color: var(--background-menu);
		display: grid;
		grid-template-columns: 1fr 1fr 1fr;
		justify-content: space-between;
		position: relative;
		padding-left: 4px;
		padding-right: 4px;
		gap: 4px;
	}

	.button-container {
		display: flex;
		justify-content: center;
		align-items: center;
		flex: 1 1 auto;
	}

	.left {
		justify-content: left;
	}

	.middle {
		justify-content: center;
	}

	.right {
		justify-content: right;
	}

	.info {
		background-color: var(--button);
		width: 6rem;
		height: 22px;
		font-size: var(--xs);
		font-family: var(--font-mono);
		display: flex;
		justify-content: center;
		align-items: center;
		border: 1px solid var(--border-secondary);
		border-left: none;
	}
</style>
