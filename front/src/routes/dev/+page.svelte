<script lang="ts">
	import { stop } from '$lib/context'
	import ControlBar from '$lib/dev/ControlBar.svelte'
	import EditorPane from '$lib/dev/EditorPane.svelte'
	import ProjectPane from '$lib/dev/panes/ProjectPane.svelte'
	import ProjectSelection from '$lib/dev/ProjectSelection.svelte'
	import Viewport from '$lib/dev/Viewport.svelte'
	import { wProjectId } from '$stores/project'
	import { wUserGeneralConfig } from '$stores/userConfig'
	import { onDestroy } from 'svelte'
	import { Pane, Splitpanes, type IPaneSizingEvent } from 'svelte-splitpanes'

	let clientHeight: number
	let clientWidth: number

	$: projectPanePct = (
		($wUserGeneralConfig.projectPanelSize / (clientWidth ?? 1000)) *
		100
	).toString()

	function resized(event: IPaneSizingEvent[]) {
		console.log(event)
	}

	// onMount(init)
	onDestroy(stop)
</script>

<div class="dev-root" bind:clientHeight bind:clientWidth>
	{#if $wProjectId}
		<Splitpanes style="height: 100%;" theme="modern-theme" on:resized>
			<!-- ---------- project pane ------------- -->
			<Pane size={projectPanePct}>
				<ProjectPane />
			</Pane>

			<Pane>
				<Splitpanes style="height: 100%;" theme="modern-theme" on:resized>
					<Pane>
						<Splitpanes horizontal theme="modern-theme" on:resized>
							<!-- ---------- viewport  ------------- -->
							<Pane size="60">
								<Viewport />
							</Pane>
							<!-- ---------- control bar   ------------- -->
							<Pane size={40} minSize={(48 / clientHeight) * 100} snapSize={5}>
								<ControlBar />
							</Pane>
						</Splitpanes>
					</Pane>
					<!-- ---------- editor   ------------- -->
					<Pane size={40} snapSize={10}>
						<EditorPane />
					</Pane>
				</Splitpanes>
			</Pane>
		</Splitpanes>
	{:else}
		<ProjectSelection />
	{/if}
</div>

<style global lang="scss">
	.dev-root {
		height: 100%;
		width: 100%;
	}

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
