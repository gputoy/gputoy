<script lang="ts">
	import { MENUKEYS } from '$core/consts'
	import context from '$core/context'
	import { loadProject, saveProject } from '$core/project'
	import { toggleUserModal, toggleUserPrefs } from '$core/util'
	import IconButton from '$lib/components/buttons/IconButton.svelte'
	import UiThemeButton from '$lib/components/buttons/UiThemeButton.svelte'
	import Debug from '$lib/components/Debug.svelte'
	import Icon from '$lib/components/Icon.svelte'
	import Key from '$lib/components/Key.svelte'
	import Logo from '$lib/components/Logo.svelte'
	import MenuItem from '$lib/dev/MenuItem.svelte'
	import UserModal from '$lib/user/UserModal.svelte'
	import UserPrefs from '$lib/user/UserPrefs.svelte'
	import { wLastInputAction, wProjectMeta, wUser } from '$stores'
	import { SvelteToast, type SvelteToastOptions } from '@zerodevx/svelte-toast'
	import { onMount } from 'svelte'

	const options: SvelteToastOptions = {}

	onMount(() => {
		const last = localStorage.getItem('last-project')
		if (last) {
			loadProject(last)
		}
	})
	/** @ts-ignore*/
	onMount(() => setTimeout(() => context.init(context), 100))
	// onDestroy(context.destroy)
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
			<b>{$wUser?.username ?? 'anonymous'}</b>&nbsp;/&nbsp;{$wProjectMeta.title}
		</div>

		<div class="nav-end nav-region">
			{#if $wLastInputAction}
				<div class="input-helper">
					<Key keycode={$wLastInputAction.code} />
					{#if $wLastInputAction.action?.ty}
						<code style="color: var(--text-accent-color);"
							>{$wLastInputAction.action.ty}</code
						>
					{/if}
				</div>
			{/if}
			<div class="navend-container">
				<IconButton
					on:click={() => saveProject(true)}
					disabled={$wUser == null}
					series="first"
				>
					<Icon name="save" stroked thick />
				</IconButton>
				<IconButton on:click={toggleUserModal} text="Fork" series="last">
					<Icon name="git-branch" stroked thick />
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
		font-size: var(--xs);
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
		border-bottom: var(--border);
		height: var(--navbar-height);
	}
	nav {
		display: grid;
		grid-template-columns: 1fr 1fr 1fr;
		background-color: var(--background-nav);
		gap: 4px;
		align-items: center;
		border-bottom: 1px solid var(--border-primary);
		padding-inline: 4px;
		height: 100%;
	}

	.input-helper {
		display: flex;
		align-items: center;
		gap: 8px;
		font-size: var(--xs);
		border: var(--border);
		border-radius: 4px;
		height: 1rem;
		padding: 3.5px 4px;
	}
</style>
