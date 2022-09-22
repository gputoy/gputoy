<script lang="ts">
	// import { init } from '$lib/context';
	import Editor from '$lib/editor/Editor.svelte';
	import ProjectPane from '$lib/editor/panes/ProjectPane.svelte';
	import { Pane, Splitpanes } from 'svelte-splitpanes';
	// onMount(init);
</script>

<div class="root">
	<Splitpanes style="height: 100%;" theme="no-splitter">
		<!-- ---------- project pane ------------- -->
		<Pane size={10} snapSize={10} maxSize={10}>
			<ProjectPane />
		</Pane>

		<Pane>
			<Splitpanes style="height: 100%;" theme="modern-theme">
				<Pane>
					<Splitpanes horizontal theme="modern-theme">
						<!-- ---------- viewport  ------------- -->
						<Pane class="canvas-container">
							<div id="canvas-root" />
						</Pane>
						<!-- ---------- other stuff   ------------- -->
						<Pane>Other stuff</Pane>
					</Splitpanes>
				</Pane>
				<!-- ---------- editor   ------------- -->
				<Pane size={40} snapSize={10}>
					<Editor />
				</Pane>
			</Splitpanes>
		</Pane>
	</Splitpanes>
</div>

<style global lang="scss">
	.canvas-container {
		padding: 2px;
	}

	#canvas-root {
		height: 100%;
		width: calc(100% - 4px);
		background-color: black;
	}
	.root {
		height: 100%;
		width: 100%;
	}

	// .splitpanes.modern-theme {
	// 	.splitpanes__pane {
	// 		background-color: var(--pure-bg);
	// 	}
	// 	.splitpanes__splitter {
	// 		background-color: var(--border-primary);
	// 		position: relative;

	// 		&:hover {
	// 			background-color: var(--border-primary);
	// 		}

	// 		&:before {
	// 			content: '';
	// 			position: absolute;
	// 			left: 0;
	// 			top: 0;
	// 			transition: opacity 0.4s;
	// 			background-color: var(--border-primay);
	// 			opacity: 0;
	// 			z-index: 1;
	// 		}
	// 		&:hover:before {
	// 			opacity: 1;
	// 		}
	// 		&.splitpanes__splitter__active {
	// 			z-index: 2; /* Fix an issue of overlap fighting with a near hovered splitter */
	// 		}
	// 		// &:first-child {
	// 		// 	cursor: auto;
	// 		// }
	// 	}
	// }
	// .modern-theme {
	// 	&.splitpanes--vertical > .splitpanes__splitter:before {
	// 		left: -3px;
	// 		right: -3px;
	// 		height: 100%;
	// 		cursor: col-resize;
	// 	}
	// 	&.splitpanes--horizontal > .splitpanes__splitter:before {
	// 		top: -3px;
	// 		bottom: -3px;
	// 		width: 100%;
	// 		cursor: row-resize;
	// 	}
	// }

	.splitpanes.modern-theme {
		.splitpanes__pane {
			background-color: var(--pure-bg);
		}
		.splitpanes__splitter {
			background-color: var(--border-primary);
			position: relative;

			&:before {
				content: '';
				position: absolute;
				left: 0;
				top: 0;
				transition: opacity 0.4s;
				background-color: var(--accent-color);
				opacity: 0;
				z-index: 1;
			}
			&:hover:before {
				opacity: 0.4;
			}
			&.splitpanes__splitter__active {
				z-index: 2; /* Fix an issue of overlap fighting with a near hovered splitter */
			}
		}
	}
	.modern-theme {
		&.splitpanes--vertical > .splitpanes__splitter:before {
			left: -3px;
			right: -3px;
			height: 100%;
			cursor: col-resize;
		}
		&.splitpanes--horizontal > .splitpanes__splitter:before {
			top: -3px;
			bottom: -3px;
			width: 100%;
			cursor: row-resize;
		}
	}

	.splitpanes.no-splitter {
		.splitpanes__pane {
			background-color: var(--pure-bg);
		}
		.splitpanes__splitter {
			background-color: var(--border-primary);
			position: relative;
		}
	}
	.no-splitter {
		&.splitpanes--horizontal > .splitpanes__splitter:before {
			width: 0.125rem;
			pointer-events: none;
			cursor: none;
		}
		&.splitpanes--vertical > .splitpanes__splitter:before {
			height: 0.125rem;
			pointer-events: none;
			cursor: none;
		}
	}
</style>
