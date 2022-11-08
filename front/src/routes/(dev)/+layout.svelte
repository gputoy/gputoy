<script lang="ts">
	import { page } from '$app/stores'
	import IconButton from '$lib/components/buttons/IconButton.svelte'
	import NavItem from '$lib/components/buttons/NavItem.svelte'
	import UiThemeButton from '$lib/components/buttons/UiThemeButton.svelte'
	import UserConfig from '$lib/user/UserConfig.svelte'
	import UserModal from '$lib/user/UserModal.svelte'
	import { wUser } from '$stores/auth'
	import { saveProject, wProjectMeta } from '$stores/project'
	import { toggleUserConfig, toggleUserModal } from '$stores/ui'
	import Icon from 'svelte-awesome'
	import codeFork from 'svelte-awesome/icons/codeFork'
	import gear from 'svelte-awesome/icons/gear'
	import save from 'svelte-awesome/icons/save'
	import user from 'svelte-awesome/icons/user'
</script>

<header>
	<nav>
		<ul id="link-container">
			<NavItem title="GPUToy" current={$page.routeId === ''} />
			<NavItem title="Browse" route="/browse" current={$page.routeId === 'browse'} />
			<NavItem title="Dev" route="/dev" current={$page.routeId === 'dev'} />
			<NavItem title="Docs" current={$page.routeId === 'docs'} />
		</ul>

		<div class="project-info">
			<p>
				{$wUser?.username ?? 'anonymous'}/{$wProjectMeta.title}
			</p>
		</div>

		<div class="navend">
			<div class="navend-container">
				<IconButton on:click={() => saveProject(true)} disabled={$wUser == null} series="first">
					<Icon data={save} />
				</IconButton>
				<IconButton on:click={toggleUserModal} text="Fork" series="last">
					<Icon data={codeFork} />
				</IconButton>
			</div>
			<UiThemeButton />

			<div class="navend-container">
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
		overflow-y: hidden;
	}
	.project-info {
		font-size: var(--sm);
	}
	.navend {
		height: 100%;
		display: flex;
		justify-content: center;
		margin-right: 0.25rem;
		gap: 0.25rem;
		align-items: center;
	}

	.navend-container {
		display: flex;
		flex-direction: row;
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
		border-bottom: 1px solid var(--border-primary);
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
