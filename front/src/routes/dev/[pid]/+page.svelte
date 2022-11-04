<script lang="ts">
	import Dev from '$lib/dev/Dev.svelte'
	import { clearProject, setProject } from '$stores/project'
	import type { ProjectResponse } from 'src/generated/types'
	import { onMount } from 'svelte'
	export let data: ProjectResponse
	onMount(() => {
		console.log(data)
		if (data) setProject(data, false)
		else clearProject()
	})
</script>

<Dev />
