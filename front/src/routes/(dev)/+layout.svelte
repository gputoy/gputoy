<script lang="ts">
	import IconButton from '$lib/components/buttons/IconButton.svelte'
	import UiThemeButton from '$lib/components/buttons/UiThemeButton.svelte'
	import Debug from '$lib/components/Debug.svelte'
	import Key from '$lib/components/Key.svelte'
	import Logo from '$lib/components/Logo.svelte'
	import { MENUKEYS } from '$lib/dev/menu/menu'
	import MenuItem from '$lib/dev/menu/MenuItem.svelte'
	import UserConfig from '$lib/user/UserConfig.svelte'
	import UserModal from '$lib/user/UserModal.svelte'
	import { wUser } from '$stores/auth'
	import { wLastInputAction } from '$stores/input'
	import { loadProject, saveProject, wProjectMeta } from '$stores/project'
	import { toggleUserConfig, toggleUserModal } from '$stores/ui'
	import { SvelteToast, type SvelteToastOptions } from '@zerodevx/svelte-toast'
	import { onMount } from 'svelte'
	import Icon from 'svelte-awesome'
	import codeFork from 'svelte-awesome/icons/codeFork'
	import gear from 'svelte-awesome/icons/gear'
	import save from 'svelte-awesome/icons/save'
	import user from 'svelte-awesome/icons/user'
	const options: SvelteToastOptions = {}

	onMount(() => {
		const last = localStorage.getItem('last-project')
		if (last) {
			loadProject(last)
		}
	})
</script>

<header>
	<nav>
		<ul class="nav-start nav-region">
			<a href="/" style="height: 18px; width: 18px; padding-inline: 4px;">
				<Logo />
			</a>
			{#each MENUKEYS as key}
				<MenuItem {key} />
			{/each}
		</ul>

		<div class="nav-mid nav-region">
			{$wUser?.username ?? 'anonymous'}&nbsp;/&nbsp;{$wProjectMeta.title}
		</div>

		<div class="nav-end nav-region">
			{#if $wLastInputAction}
				<div class="input-helper">
					<Key keycode={$wLastInputAction.code} />
					{#if $wLastInputAction.action?.ty}
						<code style="color: var(--text-accent-color);">{$wLastInputAction.action.ty}</code>
					{/if}
				</div>
			{/if}
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
	<Debug />
	<SvelteToast {options} />
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

	.nav-start {
		display: flex;
		flex-direction: row;
		align-items: center;
		gap: 0.25rem;
		margin: 0rem;
		justify-content: left;
	}
	.nav-mid {
		font-size: var(--sm);
		justify-content: center;
		text-align: center;
		min-width: max-content;
		padding-inline: 4px;
	}
	.nav-end {
		height: 100%;
		display: flex;
		justify-content: right;
		align-items: center;
		gap: 0.25rem;
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
		display: grid;
		grid-template-columns: 1fr 1fr 1fr;
		background-color: var(--nav-color);
		gap: 4px;
		align-items: center;
		border-bottom: 1px solid var(--border-primary);
		margin-inline: 4px;
	}

	.input-helper {
		display: flex;
		align-items: center;
		gap: 8px;
		font-size: var(--xs);
		border: 1px solid var(--border-secondary);
		border-radius: 4px;
		height: 1rem;
		padding: 3.5px 4px;
	}
</style>
