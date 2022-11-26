<script lang="ts">
	import context from '$core/context'
	import IconButton from '$lib/components/buttons/IconButton.svelte'
	import Icon from '$lib/components/Icon.svelte'

	let playing = false
	$: ready = $context.ready

	function handleBuild() {
		context?.build()
	}
	function handleIntrospect() {
		context?.prebuild()
	}
	function handleRender() {
		context?.render()
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
		<IconButton on:click={handleIntrospect} disabled={!ready} series="first" text="Introspect">
			<Icon name="code" stroked thick />
		</IconButton>
		<IconButton on:click={handleBuild} disabled={!ready} series="middle" text="Build">
			<Icon name="tool" stroked thick />
		</IconButton>
		<IconButton on:click={handleRender} disabled={!ready} series="last" text="Render">
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
