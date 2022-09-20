<script lang="ts">
	import Editor from '$lib/editor/Editor.svelte';
	import { onMount } from 'svelte';
	import { Pane, Splitpanes } from 'svelte-splitpanes';
	import init from '../../../pkg/gpu_wasm';

	async function _init() {
		const wasm = await init();
		const context1 = wasm.build();
		const context2 = wasm.build();
		console.log({ context1, context2 });
		wasm.print_context(context1);
		wasm.print_context(context2);
	}
	onMount(_init);
</script>

<div class="root">
	<Splitpanes theme="modern-theme" style="height: 100%;">
		<Pane size={10}>Status</Pane>
		<Pane>
			<Splitpanes horizontal theme="modern-theme">
				<Pane>
					<div id="canvas-root" />
				</Pane>
				<Pane>Other stuff</Pane>
			</Splitpanes>
		</Pane>
		<Pane>
			<Editor />
		</Pane>
	</Splitpanes>
</div>

<style global lang="scss">
	#canvas-root {
		width: 100%;
		height: 100%;
		background-color: black;
	}
	.root {
		height: 100%;
		width: 100%;
	}

	.splitpanes.modern-theme {
		.splitpanes__pane {
			background-color: var(--primary-color);
		}
		.splitpanes__splitter {
			transition: background-color 0.2s ease;
			background-color: var(--border-secondary);

			position: relative;

			&:hover {
				background-color: var(--border-primary);
			}

			// &:before {
			// 	content: '';
			// 	position: absolute;
			// 	left: 0;
			// 	top: 0;
			// 	transition: opacity 0.4s;
			// 	background-color: var(--border-primay);
			// 	opacity: 0;
			// 	z-index: 1;
			// }
			// &:hover:before {
			// 	opacity: 1;
			// }
			&:first-child {
				cursor: auto;
			}
		}
	}
	.modern-theme {
		&.splitpanes--vertical > .splitpanes__splitter {
			width: 6px;
			cursor: col-resize;
		}
		&.splitpanes--horizontal > .splitpanes__splitter {
			height: 6px;
			cursor: row-resize;
		}
	}
</style>
