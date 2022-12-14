<script lang="ts">
	import Key from '$lib/components/Key.svelte'
	import { actionPermitted, pushAction } from '$lib/core/actions'
	import { findActionBind } from '$lib/core/input'
	import { wMenuOpen } from '$stores/ui'
	import { wUserKeybinds } from '$stores/userConfig'
	import { MENUKEYS, MENU_MAP, type MenuEntry, type MenuKey } from './menu'

	export let key: MenuKey
	// When closed, skip the next call to open
	// This is so clicking a menu entry doesn't immediately reopen it
	let skipOpen = false
	let timeout: number

	$: boundActions = Object.fromEntries(
		MENU_MAP[key]
			.flat()
			.map((bind) => [bind.name, findActionBind(bind.fAction?.action, $wUserKeybinds)])
	)

	function open() {
		if (skipOpen) return
		wMenuOpen.update((m: any) => {
			MENUKEYS.forEach((k) => (m[k] = false))
			m[key] = true
			return m
		})
	}
	async function close() {
		skipOpen = true
		clearTimeout(timeout)
		timeout = window.setTimeout(() => (skipOpen = false), 10)
		wMenuOpen.update((m: any) => {
			m[key] = false
			return m
		})
	}
	function makeMenuEntryClickHandler({ fAction }: MenuEntry) {
		return function () {
			close()
			if (fAction && actionPermitted(fAction)) pushAction(fAction.action)
		}
	}
</script>

<li class="menu-button" on:click={open} on:mouseenter={open} on:mouseleave={close} on:blur={close}>
	{key.charAt(0).toUpperCase() + key.slice(1)}
	{#if $wMenuOpen[key]}
		<ul class="menu">
			{#each MENU_MAP[key] as submenu, i}
				{#each submenu as menuEntry}
					<li
						class="menu-item"
						on:click={makeMenuEntryClickHandler(menuEntry)}
						disabled={menuEntry.fAction ? !actionPermitted(menuEntry.fAction) : false}
					>
						<p>{menuEntry.name}</p>
						{#if boundActions[menuEntry.name]}
							<Key keycode={boundActions[menuEntry.name]} />
						{/if}
					</li>
				{/each}
				{#if i <= submenu.length}
					<div class="divider" />
				{/if}
			{/each}
		</ul>
	{/if}
</li>

<style>
	.menu-button {
		list-style-type: none;
		font-size: var(--xs);
		padding: 8px;
		cursor: pointer;
		position: relative;
		user-select: none;
		font-weight: bolder;
	}
	.menu-button:hover {
		background-color: var(--glass-med);
	}
	.menu {
		position: absolute;
		left: 0px;
		top: 100%;
		background-color: var(--background-menu);
		font-size: var(--xs);
		width: max-content;
		border: 1px solid var(--border-primary);
		list-style-type: none;
		padding-block: 0.25rem;
		box-shadow: var(--box-shadow);
	}
	.menu li {
		transition: none;
		padding-inline: 0.5rem;
		padding-block: 0.3rem;
		margin-block: 0.2rem;
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 1rem;
		font-weight: normal;
	}
	.menu li p {
		margin: 0px;
		display: inline;
		user-select: none;
	}
	.menu li:hover {
		background-color: var(--glass-med);
	}
	.divider {
		width: 100%;
		height: 1px;
		background-color: var(--border-primary);
		margin-block: 0.5rem;
	}
</style>
