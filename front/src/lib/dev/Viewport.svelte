<script lang="ts">
	import context from '$lib/core/context'
	let clientHeight: number
	let clientWidth: number
	let root: HTMLElement

	// resize canvas inner size, canvas style, and webgpu surface
	$: {
		let canvas = root?.children[0] as HTMLCanvasElement
		if (canvas) {
			canvas.width = clientWidth
			canvas.height = clientHeight
			canvas.setAttribute('style', `width: ${clientWidth}; height: ${clientHeight};`)
			context?.resize(clientWidth, clientHeight)
		}
	}
</script>

<div class="canvas-container" bind:clientWidth bind:clientHeight>
	<div id="canvas-root" bind:this={root} />
</div>

<style>
	* {
		--inset: 0rem;
	}

	.canvas-container {
		padding: var(--inset);
		background-color: var(--background-nav);
		width: 100%;
		height: 100%;
	}

	#canvas-root {
		width: calc(100% - calc(var(--inset) * 2));
		height: calc(100% - calc(var(--inset) * 2));
		background-color: black;
	}
</style>
