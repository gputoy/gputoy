<script lang="ts">
	import type { Panel } from '$common'
	// import { stop } from '$core/context'
	import { initKeys } from '$core/input'
	import ControlBar from '$lib/dev/ControlBar.svelte'
	import EditorPane from '$lib/dev/panes/EditorPane.svelte'
	import ProjectPane from '$lib/dev/panes/ProjectPane.svelte'
	import Viewport from '$lib/dev/Viewport.svelte'
	import { wLayout } from '$stores'
	import { onMount } from 'svelte'
	import { Pane, Splitpanes, type IPaneSizingEvent } from 'svelte-splitpanes'

	let clientHeight: number
	let clientWidth: number

	function makeResizeHandler(panel: Panel, idx: number) {
		return function resize(event: CustomEvent<IPaneSizingEvent[]>) {
			wLayout.setPanelSize(panel, event.detail[idx])
		}
	}

	$: controlBarMinSize = (32 / clientHeight) * 100
	$: projectPaneSize = $wLayout.projectPanel.show ? $wLayout.projectPanel.size : 0
	$: editorPanelSize = $wLayout.editorPanel.show ? $wLayout.editorPanel.size : 0
	$: resourcePanelSize = $wLayout.resourcePanel.show
		? $wLayout.resourcePanel.size
		: controlBarMinSize

	onMount(initKeys)
	// onDestroy(stop)
</script>

<div class="dev-root" bind:clientHeight bind:clientWidth>
	<Splitpanes
		style="height: 100%;width: 100%;"
		theme="modern-theme"
		on:resize={makeResizeHandler('projectPanel', 0)}
	>
		<!-- ---------- project pane ------------- -->
		<Pane size={projectPaneSize.toString()}>
			<ProjectPane />
		</Pane>

		<Pane size={(100 - projectPaneSize).toString()}>
			<Splitpanes
				style="height: 100%;width: 100%;"
				theme="modern-theme"
				on:resize={makeResizeHandler('editorPanel', 1)}
			>
				<Pane size={(100 - editorPanelSize).toString()}>
					<Splitpanes
						horizontal
						theme="modern-theme"
						on:resize={makeResizeHandler('resourcePanel', 1)}
					>
						<!-- ---------- viewport  ------------- -->
						<Pane size={(100 - resourcePanelSize).toString()}>
							<Viewport />
						</Pane>
						<!-- ---------- control bar   ------------- -->
						<Pane
							size={resourcePanelSize.toString()}
							minSize={controlBarMinSize.toString()}
							snapSize="5"
						>
							<ControlBar />
						</Pane>
					</Splitpanes>
				</Pane>
				<!-- ---------- editor   ------------- -->
				<Pane size={editorPanelSize.toString()} snapSize="10">
					<EditorPane />
				</Pane>
			</Splitpanes>
		</Pane>
	</Splitpanes>
</div>

<style global lang="scss">
	.dev-root {
		height: 100%;
		width: 100%;
		overflow: hidden;
	}

	.splitpanes.modern-theme {
		.splitpanes__pane {
			background-color: var(--background-content);
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
			background-color: var(--background-content);
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
