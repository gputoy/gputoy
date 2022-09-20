<script lang="ts">
	import { page } from '$app/stores';
	import IconButton from '$lib/buttons/IconButton.svelte';
	import NavItem from '$lib/buttons/NavItem.svelte';
	import UserModal from '$lib/modal/UserModal.svelte';
	import { onMount } from 'svelte';
	import FaRegMoon from 'svelte-icons/fa/FaRegMoon.svelte';
	import FaRegSun from 'svelte-icons/fa/FaRegSun.svelte';
	import FaUserCircle from 'svelte-icons/fa/FaUserCircle.svelte';
	import '../app.css';
	import { getSession } from '../stores/auth';
	import '../theme.css';
	$: {
		console.log('Route id: ', $page.routeId);
	}

	$: isNightMode = false;

	onMount(getSession);

	$: showUserModal = false;
	function onToggleNightMode() {
		window.document.body.classList.toggle('dark-mode');
		isNightMode = !isNightMode;
	}

	function onToggleUserModal() {
		showUserModal = !showUserModal;
	}

	function onHideUserModal() {
		showUserModal = false;
	}
</script>

<header>
	<nav>
		<ul id="link-container">
			<NavItem title="GPUToy" current={$page.routeId === ''} />
			<NavItem title="Browse" route="/browse" current={$page.routeId === 'browse'} />
			<NavItem title="Dev" route="/dev" current={$page.routeId === 'dev'} />
			<NavItem title="Docs" current={$page.routeId === 'docs'} />
		</ul>

		<div>
			<IconButton on:click={onToggleNightMode} class="icon-button">
				{#if isNightMode}
					<FaRegSun />
				{:else}
					<FaRegMoon />
				{/if}
			</IconButton>
			<IconButton on:click={onToggleUserModal} class="icon-button">
				<FaUserCircle />
			</IconButton>
		</div>
	</nav>
</header>
<main>
	<UserModal show={showUserModal} onHide={onHideUserModal} />
	<slot />
</main>

<style>
	main {
		flex: 1 1 auto;
		width: 100%;
		height: 100%;
		flex-grow: 1;
		display: flex;
		flex-direction: column;
		align-items: center;
		overflow-y: scroll;
	}
	div {
		height: 100%;
		display: flex;
		justify-content: center;
		margin-right: 1rem;
		align-items: center;
	}

	header {
		flex: 0 0 auto;
		border-bottom: var(--border-primary) var(--border-primary-size) solid;
		height: var(--navbar-height);
	}
	nav {
		display: flex;
		flex-direction: row;
		height: 100%;
		background-color: var(--nav-color);
		justify-content: space-between;
		align-items: center;
	}
	#link-container {
		display: flex;
		flex-direction: row;
		height: 100%;
		gap: 1rem;
		margin: 0rem;
		margin-left: 1rem;
	}
	:global(*) {
		transition: background-color 0.25s ease;
		transition: color 0.05s ease;
	}
</style>
