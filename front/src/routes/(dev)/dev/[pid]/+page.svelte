<script lang="ts">
	import { clearProject, setProject } from '$core/project'
	import type { ProjectResponse } from '$gen'
	import Dev from '$lib/workspace/Workspace.svelte'
	import { onMount } from 'svelte'
	export let data: ProjectResponse
	onMount(() => {
		if (data) setProject(data, false)
		else clearProject()
	})
</script>

<Dev />
