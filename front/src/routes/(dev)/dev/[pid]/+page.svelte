<script lang="ts">
	import type { ProjectResponse } from '$common'
	import Dev from '$lib/dev/Dev.svelte'
	import { clearProject, setProject } from '$stores'
	import { onMount } from 'svelte'
	export let data: ProjectResponse
	onMount(() => {
		if (data) setProject(data, false)
		else clearProject()
	})
</script>

<Dev />
