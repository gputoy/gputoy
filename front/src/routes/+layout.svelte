<script lang="ts">
	import CompletionProvider from '$lib/workspace/completions/CompletionProvider.svelte'
	import '../styles/app.css'

	// onMount(wUser.getSession)
</script>

<slot />
<CompletionProvider />
