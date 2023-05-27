<script lang="ts">
	import context from '$core/context'
	import { loadProject } from '$core/project'
	import Debug from '$lib/components/debug/Debug.svelte'
	import Navbar from '$lib/workspace/Navbar.svelte'
	import { SvelteToast, type SvelteToastOptions } from '@zerodevx/svelte-toast'
	import { onMount } from 'svelte'

	const options: SvelteToastOptions = {}

	onMount(() => {
		const last = localStorage.getItem('last-project')
		if (last) {
			loadProject(last)
		}
	})
	/** @ts-ignore*/
	onMount(() => setTimeout(() => context.init(context), 100))
	// onDestroy(context.destroy)
</script>

<header>
	<Navbar />
</header>
<main>
	<Debug />
	<SvelteToast {options} />
	<slot />
</main>

<style>
	header {
		z-index: 2;
		flex: 0 0 auto;
		height: var(--navbar-height);
		padding: 2px;
		background-color: var(--border-primary);
	}
	main {
		height: 100vh;
		display: flex;
		align-items: center;
		overflow-y: hidden;
		padding: 2px;
		padding-top: 0px;
		background-color: var(--border-primary);
	}
</style>
