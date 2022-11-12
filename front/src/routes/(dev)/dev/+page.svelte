<script lang="ts">
	import Dev from '$lib/dev/Dev.svelte'
	import ProjectSelection from '$lib/dev/ProjectSelection.svelte'
	import { wProjectId } from '$stores/project'
	import { toast } from '@zerodevx/svelte-toast'
	import { onMount } from 'svelte'

	onMount(() => toast.push('Hello hi hi we are kewl', { duration: 60000 }))
</script>

{#if $wProjectId}
	<Dev />
{:else}
	<ProjectSelection />
{/if}
