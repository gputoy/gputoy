<script lang="ts">
	import { initKeys } from '$core/keys'
	import {
		applySplitpaneEvent,
		dPaneSizes,
		wUpdatingWindowSize,
		wWindowHeight,
		wWindowWidth
	} from '$core/layout'
	import PreferenceTray from '$lib/components/preferences/PreferenceTray.svelte'
	import EditorPane from '$lib/workspace/panes/EditorPane.svelte'
	import ProjectPane from '$lib/workspace/panes/ProjectPane.svelte'
	import ViewportPane from '$lib/workspace/panes/ViewportPane.svelte'
	import { onMount } from 'svelte'
	import { Pane, Splitpanes } from 'svelte-splitpanes'
	import CompletionProvider from './completions/CompletionProvider.svelte'
	import ControlPane from './panes/ControlPane.svelte'

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
				<!-- ---------- Control ------------- -->
				<Pane
					size={$dPaneSizes?.controlPanePct}
					minSize={$dPaneSizes?.controlBarMinSize}
				>
					<ControlPane />
				</Pane>
			</Splitpanes>
		</Pane>
		<!-- ---------- editor   ------------- -->
		<Pane class={clazz} size={$dPaneSizes?.editorPanePct}>
			<EditorPane />
		</Pane>
	</Splitpanes>
	<PreferenceTray />
	<CompletionProvider />
</div>

<style global lang="scss">
	.dev-root {
		height: 100%;
		width: 100%;
		overflow: hidden;
	}
</style>
