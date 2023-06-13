<script lang="ts">
	import { pushAction } from '$core/actions'
	import context from '$core/context'
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
	function handlePlayPause() {
		pushAction({ ty: 'playPause' })
	}
</script>

<div id="controlbar-container">
	<div class="left button-container">
		<button on:click={() => pushAction({ ty: 'toggleUi', c: 'terminal' })}>
			<Icon name="command" stroked thick />
		</button>
	</div>
	<div class="middle button-container">
		<button class="right-flat" on:click={handlePlayPause}>
			{#if playing}
				<Icon name="pause" class="sm" filled />
			{:else}
				<Icon name="play" class="sm" filled />
			{/if}
		</button>
		<button class="flat">
			<Icon name="square" class="sm" filled />
		</button>
		<div class="info">0.0 fps</div>
		<button class="left-flat">
			<Icon name="circle" class="sm" filled />
		</button>
	</div>

	<div class="right button-container">
		<button on:click={handleIntrospect} disabled={!ready} class="right-flat">
			<Icon name="code" thick />
		</button>
		<button on:click={handleBuild} disabled={!ready} class="flat">
			<Icon name="tool" thick />
		</button>
		<button on:click={handleRender} disabled={!ready} class="left-flat">
			<Icon name="monitor" thick />
		</button>
	</div>
</div>

<style>
	#controlbar-container {
		flex: 0 0 auto;
		border-radius: var(--pane-radius);
		border-bottom: var(--border);
		height: var(--nav-height);
		display: grid;
		background-color: var(--background-nav);
		grid-template-columns: 1fr 1fr 1fr;
		justify-content: space-between;
		position: relative;
		gap: var(--nav-gap);
		padding-inline: var(--nav-gap);
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
		background-color: var(--input-color);
		width: 6rem;
		height: var(--md-input);
		font-size: var(--sm);
		color: var(--text-accent-color);
		font-family: var(--font-mono);
		font-weight: bold;
		display: flex;
		justify-content: center;
		align-items: center;
	}
</style>
