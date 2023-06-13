<script lang="ts">
	import context from '$core/context'
	import Console from '$lib/workspace/Console.svelte'
	let clientHeight: number
	let clientWidth: number
	let root: HTMLElement

	// resize canvas inner size, canvas style, and webgpu surface
	$: {
		let canvas = root?.children[0] as HTMLCanvasElement
		if (canvas) {
			canvas.width = clientWidth
			canvas.height = clientHeight
			canvas.setAttribute(
				'style',
				`width: ${clientWidth}; height: ${clientHeight};`
			)
			context?.resize(clientWidth, clientHeight)
		}
	}
</script>

<div class="canvas-container pane" bind:clientWidth bind:clientHeight>
	<div id="canvas-root" bind:this={root} />
	<Console />
</div>

<style>
	* {
		--inset: 0rem;
	}

	.canvas-container {
		padding: var(--inset);
		width: 100%;
		height: 100%;
		display: relative;
		border-radius: var(--pane-radius);
		box-sizing: border-box;
		border-bottom: var(--border);
		inset-block-end: var(--gap);
	}

	#canvas-root {
		width: calc(100% - calc(var(--inset) * 2));
		height: calc(100% - calc(var(--inset) * 2));
		background-color: black;
		border-radius: var(--pane-radius);
	}
</style>
