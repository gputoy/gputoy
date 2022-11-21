<script lang="ts">
	import type { ProjectResponse } from '$common'
	import { clearProject, setProject } from '$core/project'
	import Dev from '$lib/dev/Dev.svelte'
	import { onMount } from 'svelte'
	export let data: ProjectResponse
	onMount(() => {
		if (data) setProject(data, false)
		else clearProject()
	})
</script>

<Dev />
