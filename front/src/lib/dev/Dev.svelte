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
	$: projectPaneSize = $wLayout.projectPanel.show
		? $wLayout.projectPanel.size
		: 0
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
		<Pane size={projectPaneSize}>
			<ProjectPane />
		</Pane>

		<Pane size={100 - projectPaneSize}>
			<Splitpanes
				style="height: 100%;width: 100%;"
				theme="modern-theme"
				on:resize={makeResizeHandler('editorPanel', 1)}
			>
				<Pane size={100 - editorPanelSize}>
					<Splitpanes
						horizontal
						theme="modern-theme"
						on:resize={makeResizeHandler('resourcePanel', 1)}
					>
						<!-- ---------- viewport  ------------- -->
						<Pane size={100 - resourcePanelSize}>
							<Viewport />
						</Pane>
						<!-- ---------- control bar   ------------- -->
						<Pane
							size={resourcePanelSize}
							minSize={controlBarMinSize}
							snapSize={5}
						>
							<ControlBar />
						</Pane>
					</Splitpanes>
				</Pane>
				<!-- ---------- editor   ------------- -->
				<Pane size={editorPanelSize} snapSize={10}>
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
</style>
