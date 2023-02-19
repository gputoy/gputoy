<script lang="ts">
	import { wWorkerInternal } from '$monaco/wgsl/wgslMode'
	import {
		dCanModifyProject,
		dProject,
		dUserPrefs,
		wDebugPanel,
		wMenuOpen,
		wPrebuildDirty,
		wPrebuildResult,
		wUser,
		wUserModalOpen,
		wUserPrefsOpen,
		wWorkerData
	} from '$stores'
	/** @ts-ignore */
	import { JsonView } from '@zerodevx/svelte-json-view'
	import IconButton from './buttons/IconButton.svelte'

	$: json = {
		canModifyProject: $dCanModifyProject,
		user: $wUser,
		config: $dUserPrefs,
		project: $dProject,
		ui: {
			userModalOpen: $wUserModalOpen,
			UserPrefsOpen: $wUserPrefsOpen,
			menuOpen: $wMenuOpen
		},
		prebuild: {
			dirty: $wPrebuildDirty,
			result: $wPrebuildResult
		},
		monacoWorker: $wWorkerInternal,
		workerData: $wWorkerData
	}
</script>

{#if $wDebugPanel}
	<div class="debug-container">
		<div>
			<IconButton
				on:click={async () => {
					wWorkerData.set(await wWorkerInternal.poll())
				}}>Reload worker</IconButton
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
