<script lang="ts">
	import { wUser } from '$stores'
	import { onMount } from 'svelte'
	import '../app.css'
	import '../theme.css'

	onMount(wUser.getSession)
</script>

<slot />
