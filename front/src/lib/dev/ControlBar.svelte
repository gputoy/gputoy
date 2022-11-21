<script lang="ts">
	import { build, prebuild, render as context_render } from '$core/context'
	import { getProject } from '$core/project'
	import IconButton from '$lib/components/buttons/IconButton.svelte'
	import Icon from '$lib/components/Icon.svelte'
	import { wFiles } from '$stores'
	import { get } from 'svelte/store'

	let x = context_render
	let playing = false

	function handleBuild() {
		build(getProject())
	}
	function handleIntrospect() {
		prebuild(get(wFiles))
	}
</script>

<div id="controlbar-container">
	<div class="left button-container">
		<IconButton smallIcons>
			<Icon name="square" />
		</IconButton>
	</div>
	<div class="middle button-container">
		<IconButton series="first" smallIcons>
			{#if playing}
				<Icon name="pause" />
			{:else}
				<Icon name="play" />
			{/if}
		</IconButton>
		<IconButton series="middle" smallIcons>
			<Icon name="square" />
		</IconButton>
		<div class="info">0.0 fps</div>
		<IconButton series="last" smallIcons>
			<Icon name="circle" />
		</IconButton>
	</div>

	<div class="right button-container">
		<IconButton on:click={handleIntrospect} series="first" text="Introspect">
			<Icon name="code" stroked thick />
		</IconButton>
		<IconButton on:click={handleBuild} series="middle" text="Build">
			<Icon name="tool" stroked thick />
		</IconButton>
		<IconButton on:click={context_render} series="last" text="Render">
			<Icon name="monitor" stroked thick />
		</IconButton>
	</div>
</div>

<style>
	#controlbar-container {
		width: calc(100% - 8px);
		border-bottom: var(--border);
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
		border: var(--border);
		border-left: none;
	}
</style>
