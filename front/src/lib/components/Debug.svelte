<script lang="ts">
	import { wUser } from '$stores/auth'
	import { dCanModifyProject, dProject } from '$stores/project'
	import { wDebugPanel, wMenuOpen, wUserConfigOpen, wUserModalOpen } from '$stores/ui'
	import { dUserConfig } from '$stores/userConfig'
	/** @ts-ignore */
	import { JsonView } from '@zerodevx/svelte-json-view'
	$: json = {
		canModifyProject: $dCanModifyProject,
		user: $wUser,
		config: $dUserConfig,
		project: $dProject,
		ui: {
			userModalOpen: $wUserModalOpen,
			userConfigOpen: $wUserConfigOpen,
			menuOpen: $wMenuOpen
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
		border: 1px solid var(--border-primary);
	}
</style>
