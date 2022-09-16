<script lang="ts">
	import vars from '$lib/vars';
	let username_or_email: string = '';
	let password: string = '';
	let invalidIdentifier = false;
	let invalidPassword = false;
	$: unauthorized = false;

	$: {
		unauthorized = false;
	}

	$: if (username_or_email || password) {
		invalidIdentifier = false;
		invalidPassword = false;
	}
	async function onSubmit() {
		if (isIdentifierInvalid(username_or_email)) invalidIdentifier = true;
		if (isPasswordInvalid(password)) invalidPassword = true;
		if (invalidIdentifier || invalidPassword) return;
		const response = await fetch(vars.API_PATH + 'login', {
			method: 'POST',
			headers: {
				'Content-Type': 'application/x-www-form-urlencoded'
			},
			body: new URLSearchParams({
				username_or_email,
				password
			}),
			credentials: 'include'
		});
		const json = await response.text();
		// unauthorized
		if (response.status == 401) {
			unauthorized = true;
			return;
		}
		console.log('Response: ', json);
		//window.location = '/profile';
	}

	function isIdentifierInvalid(username: string): boolean {
		return username.length < 3 || username.length > 40;
	}
	function isPasswordInvalid(password: string): boolean {
		return password.length < 8 || password.length > 40;
	}
</script>

<form on:submit|preventDefault={onSubmit} class="form" action="#" method="post">
	<span><strong>Log in</strong> to gputoy</span>
	<input
		type="text"
		name="username"
		placeholder="Username/email"
		autocomplete="username"
		bind:value={username_or_email}
	/>
	{#if invalidIdentifier}
		<div class="invalid">Username/email is invalid</div>
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
	<div>
		<button type="submit"> Log in </button>
		<button>Forgot password</button>
	</div>

	{#if unauthorized}
		<div class="invalid">Invalid credentials</div>
	{/if}
</form>

<style>
	* {
		border: none;
		border-radius: 4px;
	}
	.form {
		background-color: var(--primary-color);
		padding: 12px;
		display: flex;
		flex-direction: column;
		max-width: 12rem;
		gap: 1rem;
		box-shadow: 0 0 10px rgba(0, 0, 0, 0.219);
		align-items: center;
	}
	input {
		padding: 12px;
		box-shadow: inset 0 0 10px rgba(0, 0, 0, 0.219);
	}
	button {
		padding: 8px;
		font-weight: bold;
	}
	.invalid {
		color: red;
	}
</style>
