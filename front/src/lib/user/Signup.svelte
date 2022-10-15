<script lang="ts">
	import vars from '$lib/vars'
	let username: string = ''
	let password: string = ''
	let email: string = ''

	$: invalidUsername = isUsernameInvalid(username)
	$: invalidEmail = isEmailInvalid(email)
	$: invalidPassword = isPasswordInvalid(password)
	async function onSubmit() {
		const response = await fetch(vars.API_PATH + 'signup', {
			method: 'POST',
			headers: {
				'Content-Type': 'application/x-www-form-urlencoded'
			},
			body: new URLSearchParams({
				username,
				email,
				password
			})
		})
		const json = await response.json()
		console.log('Response: ', json)
	}

	function isUsernameInvalid(username: string): boolean {
		return username.length < 3 || username.length > 30
	}
	function isEmailInvalid(email: string): boolean {
		return !/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(email)
	}
	function isPasswordInvalid(password: string): boolean {
		return password.length < 8 || password.length > 40
	}
</script>

<form on:submit|preventDefault={onSubmit} class="form" action="#" method="post">
	<input type="email" name="email" placeholder="Email" autocomplete="email" bind:value={email} />
	{#if invalidEmail}
		<div class="invalid">Email is invalid</div>
	{/if}
	<input
		type="text"
		name="username"
		placeholder="Username"
		autocomplete="username"
		bind:value={username}
	/>
	{#if invalidUsername}
		<div class="invalid">Username is invalid</div>
	{/if}
	<input
		type="password"
		name="password"
		placeholder="Password"
		autocomplete="new-password"
		bind:value={password}
	/>
	{#if invalidPassword}
		<div class="invalid">Password is invalid</div>
	{/if}
	<button type="submit"> Submit </button>
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
		border: 1px black dashed;
		border-radius: 4px;
		padding: 4px;
	}
	.invalid {
		color: red;
	}
</style>
