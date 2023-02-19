<script lang="ts">
	import { page } from '$app/stores'
	import { toggleUserModal, toggleUserPrefs } from '$core/util'
	import IconButton from '$lib/components/buttons/IconButton.svelte'
	import NavItem from '$lib/components/buttons/NavItem.svelte'
	import UiThemeButton from '$lib/components/buttons/UiThemeButton.svelte'
	import Icon from '$lib/components/Icon.svelte'
	import Logo from '$lib/components/Logo.svelte'
	import UserModal from '$lib/user/UserModal.svelte'
	import UserPrefs from '$lib/user/UserPrefs.svelte'
	import { wUser } from '$stores'
</script>

<header>
	<nav>
		<ul id="link-container">
			<div class="logo-container">
				<Logo />
				<NavItem
					title="GPUToy"
					current={$page.route.id === ''}
					style="font-weight: bold"
				/>
			</div>
			<NavItem
				title="Browse"
				route="/browse"
				current={$page.route.id === 'browse'}
			/>
			<NavItem title="Dev" route="/dev" current={$page.route.id === 'dev'} />
			<NavItem title="Docs" current={$page.route.id === 'docs'} />
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
						<Icon name="user" stroked thick />
					</IconButton>
				{:else}
					<IconButton on:click={toggleUserModal} text="Sign in" series="first">
						<Icon name="user" stroked thick />
					</IconButton>
				{/if}
				<IconButton on:click={toggleUserPrefs} series="last">
					<Icon name="settings" stroked thick />
				</IconButton>
			</div>
		</div>
	</nav>
</header>
<main>
	<UserModal />
	<UserPrefs />
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
		border-bottom: var(--border);
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
		border-bottom: var(--border);
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
</style>
