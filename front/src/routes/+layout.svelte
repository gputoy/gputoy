<script lang="ts">
	import { onMount } from 'svelte'
	import '../app.css'
	import { getSession } from '../stores/auth'
	import '../theme.css'

	onMount(getSession)
</script>

<slot />
