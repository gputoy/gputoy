<script lang="ts">
	import { wUser } from '$stores'
	import { onMount } from 'svelte'

	import '../styles/app.css'
	import '../styles/split-panes.css'
	import '../styles/theme.css'

	onMount(wUser.getSession)
</script>

<slot />
