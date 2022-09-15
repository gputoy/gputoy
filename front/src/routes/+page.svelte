<script lang="ts">
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

<button on:click={handleClick}> click mee</button>

<form class="form" action="http://localhost:8080/signup">
	<input type="email" name="email" placeholder="Email" autocomplete="email" />
	<input type="text" name="username" placeholder="Username" autocomplete="username" />
	<input type="password" name="password" placeholder="Password" autocomplete="new-password" />
	<input type="submit" />
</form>

<style>
	.form {
		padding: 1rem;
		border: 1px red solid;
		display: flex;
		flex-direction: column;
		max-width: 12rem;
		gap: 1rem;
	}
	input {
		border: 3px blue solid;
	}
</style>
