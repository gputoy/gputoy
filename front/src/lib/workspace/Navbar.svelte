<script lang="ts">
	import { pushAction } from '$core/actions'
	import { MENUKEYS } from '$core/consts'
	import { rLastInputAction } from '$core/keys'
	import { saveProject } from '$core/project'
	import { rTheme, toggleTheme } from '$core/theme'
	import Icon from '$lib/components/Icon.svelte'
	import Key from '$lib/components/Key.svelte'
	import Logo from '$lib/components/Logo.svelte'
	import MenuItem from '$lib/workspace/MenuItem.svelte'
	import { wProjectMeta, wUser } from '$stores'

	function toggleUserModal() {
		pushAction({ ty: 'toggleUi', c: 'user' })
	}
	function toggleUserPrefs() {
		pushAction({ ty: 'toggleUi', c: 'preferences' })
	}
</script>

<nav>
	<ul class="nav-start">
		<a href="/" style="height: 18px; width: 18px; padding-inline: 4px;">
			<Logo />
		</a>
		{#each MENUKEYS as key}
			<MenuItem {key} />
		{/each}
	</ul>

	<div class="nav-mid">
		<b>{$wUser?.username ?? 'anonymous'}</b>&nbsp;/&nbsp;{$wProjectMeta.title}
	</div>

	<div class="nav-end">
		{#if $rLastInputAction}
			<div class="input-helper">
				<Key keycode={$rLastInputAction.code} />
				{#if $rLastInputAction.action?.ty}
					<code style="color: var(--text-accent-color);"
						>{$rLastInputAction.action.ty}</code
					>
				{/if}
			</div>
		{/if}
		<div class="navend-container">
			<button
				class="right-flat"
				on:click={() => saveProject(true)}
				disabled={$wUser == null}
			>
				<Icon name="save" stroked thick />
			</button>

			<button class="left-flat pi4" on:click={toggleUserModal}>
				<Icon name="git-branch" stroked thick />
				Fork
			</button>
		</div>

		<button on:click={toggleTheme}>
			{#if $rTheme == 'dark'}
				<Icon name="sun" stroked thick />
			{:else}
				<Icon name="moon" stroked thick />
			{/if}
		</button>

		<div class="navend-container">
			{#if $wUser}
				<button on:click={toggleUserModal}>
					<Icon name="user" stroked thick />
					{$wUser.fullName ?? $wUser.username}
				</button>
			{:else}
				<button class="right-flat pi4" on:click={toggleUserModal}>
					<Icon name="user" stroked thick />
					Sign in
				</button>
			{/if}
			<button class="left-flat" on:click={toggleUserPrefs}>
				<Icon name="settings" stroked thick />
			</button>
		</div>
	</div>
</nav>

<style>
	nav {
		display: grid;
		grid-template-columns: 1fr 1fr 1fr;
		background-color: var(--background-nav);
		gap: var(--nav-gap);
		align-items: center;
		height: 100%;
		box-sizing: content-box;
		border-radius: var(--pane-radius);
		padding-inline: var(--nav-gap);
	}
	.nav-start {
		display: flex;
		flex-direction: row;
		align-items: center;
		margin: 0rem;
		gap: var(--nav-gap);
		justify-content: left;
	}
	.nav-mid {
		font-size: var(--md);
		justify-content: center;
		text-align: center;
		min-width: max-content;
	}
	.nav-end {
		height: 100%;
		display: flex;
		justify-content: right;
		align-items: center;
		gap: var(--nav-gap);
	}

	.navend-container {
		display: flex;
		flex-direction: row;
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
