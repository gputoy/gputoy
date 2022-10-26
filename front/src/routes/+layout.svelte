<script lang="ts">
	import { page } from '$app/stores'
	import IconButton from '$lib/buttons/IconButton.svelte'
	import NavItem from '$lib/buttons/NavItem.svelte'
	import UiThemeButton from '$lib/buttons/UiThemeButton.svelte'
	import UserModal from '$lib/modal/UserModal.svelte'
	import { wProjectId, wProjectMeta } from '$stores/project'
	import { onMount } from 'svelte'
	import Icon from 'svelte-awesome'
	import codeFork from 'svelte-awesome/icons/codeFork'
	import user from 'svelte-awesome/icons/user'
	import '../app.css'
	import { getSession, wUser } from '../stores/auth'
	import '../theme.css'
	$: {
		console.log('Route id: ', $page.routeId)
	}

	onMount(getSession)

	$: showUserModal = false

	function onToggleUserModal() {
		showUserModal = !showUserModal
	}

	function onHideUserModal() {
		showUserModal = false
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

		{#if $wProjectId}
			<div class="project-info">
				<p>
					{$wUser?.username ?? 'anonymous'}/{$wProjectMeta.title}
				</p>
			</div>
		{/if}

		<div class="navend">
			<IconButton on:click={onToggleUserModal} text="Fork">
				<Icon data={codeFork} />
			</IconButton>
			<UiThemeButton />

			{#if $wUser}
				<IconButton on:click={onToggleUserModal} text={$wUser.fullName ?? $wUser.username}>
					<Icon data={user} />
				</IconButton>
			{:else}
				<IconButton on:click={onToggleUserModal} text="Sign in">
					<Icon data={user} />
				</IconButton>
			{/if}
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
	.project-info {
		font-size: var(--sm);
	}
	.navend {
		height: 100%;
		display: flex;
		justify-content: center;
		gap: 0.25rem;
		margin-right: 0.25rem;
		align-items: center;
	}

	header {
		z-index: 2;
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
		gap: 0.5rem;
		margin: 0rem;
		margin-left: 0.25rem;
	}
	:global(*) {
		transition: background-color 0.25s ease;
		transition: color 0.05s ease;
	}
</style>
