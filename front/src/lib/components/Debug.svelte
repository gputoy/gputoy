<script lang="ts">
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
		wUserPrefsOpen
	} from '$stores'
	/** @ts-ignore */
	import { JsonView } from '@zerodevx/svelte-json-view'
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
		}
	}
</script>

{#if $wDebugPanel}
	<div class="debug-container">
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
		overflow-y: scroll;
		background-color: var(--background-content);
		padding: 1rem;
		border: var(--border);
	}
</style>
