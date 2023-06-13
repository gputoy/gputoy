<script lang="ts">
	import { wUser } from '$stores'
	import { onMount } from 'svelte'

	import CompletionProvider from '$lib/workspace/completions/CompletionProvider.svelte'
	import '../styles/app.css'

	onMount(wUser.getSession)
</script>

<slot />
<CompletionProvider />
