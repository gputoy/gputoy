<script lang="ts">
	import ProjectSelection from '$lib/workspace/ProjectSelection.svelte'
	import Dev from '$lib/workspace/Workspace.svelte'
	import { wProjectId } from '$stores'
</script>

{#if $wProjectId}
	<Dev />
{:else}
	<ProjectSelection />
{/if}
