<script lang="ts">
	import Login from '$lib/forms/Login.svelte';
	import Signup from '$lib/forms/Signup.svelte';
	import vars from '$lib/vars';
	// import { onMount } from 'svelte';
	// import init from '../../pkg/gpu_wasm.js';

	// onMount(() => {
	// 	async function __init() {
	// 		const wasm = await init();

	// 		let context;
	// 		try {
	// 			context = wasm.build();
	// 		} catch (e) {
	// 			console.error('context build ended with error: ', e);
	// 			return;
	// 		}
	// 		console.log('context created successfully, ', context);

	// 		wasm.print_context(context);
	// 	}
	// 	__init();
	// });

	async function handleClick() {
		let res = await fetch(vars.API_PATH + 'test', { method: 'GET' });
		let data = await res.text();
		console.log(data);
	}
</script>

<svelte:head>
	<title>gputoy</title>
	<meta name="description" content="Gputoy" />
</svelte:head>
<div id="main">
	<Signup />
	<Login />
</div>

<style>
	#main {
		display: flex;
		flex-direction: row;
		justify-content: center;
		gap: 2rem;
	}
</style>
