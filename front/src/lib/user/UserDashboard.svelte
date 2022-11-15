<script lang="ts">
	import IconButton from '$lib/components/buttons/IconButton.svelte'
	import { logout } from '$stores/auth'
</script>

<div id="root">
	<div id="head">
		<h2>Settings</h2>
		<IconButton text="Sign out" size="md" on:click={logout} />
	</div>
	<div id="tray" />
</div>

<style>
	#root {
		width: 600px;
		height: fit-content;
		background-color: var(--background-alt);
		box-shadow: 5px 10px 10px rgba(0, 0, 0, 0.1);
		border: var(--border2);
	}

	h2 {
		color: var(--text-accent-color);
	}

	#head {
		padding: 20px;
		height: 5em;
		display: flex;
		justify-content: space-between;
		align-items: center;
	}

	#tray {
		height: 40px;
		background-color: var(--background-menu);
	}
</style>
