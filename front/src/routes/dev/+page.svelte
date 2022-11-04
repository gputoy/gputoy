<script lang="ts">
	import Dev from '$lib/dev/Dev.svelte'
	import ProjectSelection from '$lib/dev/ProjectSelection.svelte'
	import { wProjectId } from '$stores/project'
</script>

{#if $wProjectId}
	<Dev />
{:else}
	<ProjectSelection />
{/if}
