<script lang="ts">
	import { login } from '$stores/auth'

	let username_or_email: string = ''
	let password: string = ''
	let invalidIdentifier = false
	let invalidPassword = false
	$: unauthorized = false

	$: {
		unauthorized = false
	}

	$: if (username_or_email || password) {
		invalidIdentifier = false
		invalidPassword = false
	}
	async function onSubmit() {
		if (isIdentifierInvalid(username_or_email)) invalidIdentifier = true
		if (isPasswordInvalid(password)) invalidPassword = true
		if (invalidIdentifier || invalidPassword) return
		const response = await login(username_or_email, password)
	}

	function isIdentifierInvalid(username: string): boolean {
		return username.length < 3 || username.length > 40
	}
	function isPasswordInvalid(password: string): boolean {
		return password.length < 8 || password.length > 40
	}
</script>

<form on:submit|preventDefault={onSubmit} class="form" action="#" method="post">
	<span><strong>Log in</strong> to gputoy</span>
	<input
		id="login-focus"
		type="text"
		name="username"
		placeholder="Username"
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
	</div>

	{#if unauthorized}
		<div class="invalid">Invalid credentials</div>
	{/if}
</form>

<style>
	strong {
		color: var(--text-important);
	}
	span {
		font-size: var(--lg);
		color: var(--text-accent-color);
		font-family: var(--font-heading);
	}

	.form {
		/* border-radius: 31px;
		background: #d8dfe4;
		box-shadow: 8px 8px 43px #b8bec2, -8px -8px 43px #f8ffff; */
		background-color: var(--background-alt);
		border: var(--border);
		padding: 1rem;
		display: flex;
		flex-direction: column;
		max-width: 12rem;
		gap: 2rem;
		box-shadow: var(--card-shadow);
		align-items: center;
	}
	button {
		padding: 1rem;
		font-weight: bold;
		font-size: var(--md);
		background-color: var(--accent-color);
		color: var(--text-accent-color);
	}
	.invalid {
		color: red;
	}
</style>
