<script lang="ts">
	import Dev from '$lib/dev/Dev.svelte'
	import ProjectSelection from '$lib/dev/ProjectSelection.svelte'
	import { wProjectId } from '$stores'
</script>

{#if $wProjectId}
	<Dev />
{:else}
	<ProjectSelection />
{/if}
