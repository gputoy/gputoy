<script lang="ts">
	// import { stop } from '$core/context'
	import { initKeys } from '$core/input'
	import {
		applySplitpaneEvent,
		dPaneSizes,
		wUpdatingWindowSize,
		wWindowHeight,
		wWindowWidth
	} from '$core/layout'
	import ControlBar from '$lib/dev/ControlBar.svelte'
	import EditorPane from '$lib/dev/panes/EditorPane.svelte'
	import ProjectPane from '$lib/dev/panes/ProjectPane.svelte'
	import ViewportPane from '$lib/dev/panes/ViewportPane.svelte'
	import { onMount } from 'svelte'
	import { Pane, Splitpanes } from 'svelte-splitpanes'

	$: clazz = $wUpdatingWindowSize ? 'no-transition' : undefined

	onMount(initKeys)
	// onDestroy(stop)
</script>

<div
	class="dev-root"
	bind:clientHeight={$wWindowHeight}
	bind:clientWidth={$wWindowWidth}
>
	<Splitpanes
		style="height: 100%;width: 100%;"
		theme="modern-theme"
		on:resize={applySplitpaneEvent}
		pushOtherPanes={false}
	>
		<!-- ---------- project pane ------------- -->
		<Pane class={clazz} size={$dPaneSizes?.projectPanePct} maxSize={40}>
			<ProjectPane />
		</Pane>

		<Pane class={clazz} size={$dPaneSizes?.centerPanePct}>
			<Splitpanes
				horizontal
				theme="modern-theme"
				on:resize={applySplitpaneEvent}
			>
				<!-- ---------- viewport  ------------- -->
				<Pane class={clazz} size={$dPaneSizes?.viewportPanePct}>
					<ViewportPane />
				</Pane>
				<!-- ---------- control bar   ------------- -->
				<Pane
					size={$dPaneSizes?.resourcePanePct}
					minSize={$dPaneSizes?.controlBarMinSize}
					snapSize={5}
				>
					<ControlBar />
				</Pane>
			</Splitpanes>
		</Pane>
		<!-- ---------- editor   ------------- -->
		<Pane class={clazz} size={$dPaneSizes?.editorPanePct} snapSize={10}>
			<EditorPane />
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
