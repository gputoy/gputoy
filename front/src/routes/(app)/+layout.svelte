<script lang="ts">
	import { page } from '$app/stores'
	import IconButton from '$lib/components/buttons/IconButton.svelte'
	import NavItem from '$lib/components/buttons/NavItem.svelte'
	import UiThemeButton from '$lib/components/buttons/UiThemeButton.svelte'
	import Logo from '$lib/components/Logo.svelte'
	import UserConfig from '$lib/user/UserConfig.svelte'
	import UserModal from '$lib/user/UserModal.svelte'
	import { wUser } from '$stores/auth'
	import { toggleUserConfig, toggleUserModal } from '$stores/ui'
	import Icon from 'svelte-awesome'
	import gear from 'svelte-awesome/icons/gear'
	import user from 'svelte-awesome/icons/user'
</script>

<header>
	<nav>
		<ul id="link-container">
			<div class="logo-container">
				<Logo />
				<NavItem title="GPUToy" current={$page.routeId === ''} style="font-weight: bold" />
			</div>
			<NavItem title="Browse" route="/browse" current={$page.routeId === 'browse'} />
			<NavItem title="Dev" route="/dev" current={$page.routeId === 'dev'} />
			<NavItem title="Docs" current={$page.routeId === 'docs'} />
		</ul>

		<div class="navend">
			<UiThemeButton />
			<div style="display: flex; flex-direction: row;">
				{#if $wUser}
					<IconButton
						on:click={toggleUserModal}
						text={$wUser.fullName ?? $wUser.username}
						series="first"
					>
						<Icon data={user} />
					</IconButton>
				{:else}
					<IconButton on:click={toggleUserModal} text="Sign in" series="first">
						<Icon data={user} />
					</IconButton>
				{/if}
				<IconButton on:click={toggleUserConfig} series="last">
					<Icon data={gear} />
				</IconButton>
			</div>
		</div>
	</nav>
</header>
<main>
	<UserModal />
	<UserConfig />
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
	}

	.navend {
		height: 100%;
		display: flex;
		justify-content: center;
		gap: 0.5rem;
		margin-right: 0.25rem;
		align-items: center;
	}
	.logo-container {
		display: flex;
		align-items: center;
		margin-right: 4rem;
	}

	header {
		z-index: 2;
		flex: 0 0 auto;
		border-bottom: var(--border-primary) var(--border-primary-size) solid;
		height: 48px;
	}
	nav {
		position: relative;
		display: flex;
		flex-direction: row-reverse;
		height: 100%;
		background-color: var(--nav-color);
		justify-content: space-between;
		align-items: center;
		border-bottom: 1px solid var(--border-primary);
	}
	#link-container {
		position: absolute;
		left: 50%;
		translate: -50%;
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
