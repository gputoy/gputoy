<script lang="ts">
	import { wWorkerInternal } from '$monaco/wgsl/wgslMode'
	import {
		dCanModifyProject,
		dProject,
		wPrebuildDirty,
		wPrebuildResult,
		wUser,
		wWorkerData
	} from '$stores'
	/** @ts-ignore */
	import { dLayout, rDebugOpen } from '$core/layout'
	import { dPreferences } from '$core/preferences'
	import { JsonView } from '@zerodevx/svelte-json-view'

	$: json = {
		canModifyProject: $dCanModifyProject,
		user: $wUser,
		preferences: $dPreferences,
		project: $dProject,
		layout: $dLayout,
		prebuild: {
			dirty: $wPrebuildDirty,
			result: $wPrebuildResult
		},
		monacoWorker: $wWorkerInternal,
		workerData: $wWorkerData
	}
</script>

{#if $rDebugOpen}
	<div class="debug-container">
		<div>
			<button
				on:click={async () => {
					wWorkerData.set(await wWorkerInternal.poll())
				}}>Reload worker</button
			>
		</div>
		<JsonView {json} depth={2} />
	</div>
{/if}

<style>
	.debug-container {
		position: absolute;
		top: 50%;
		left: 50%;
		z-index: 10;
		translate: -50% -50%;
		max-height: 80%;
		max-width: 80%;
		overflow-y: scroll;
		background-color: var(--background-content);
		padding: 1rem;
		border: var(--border);
	}
</style>
