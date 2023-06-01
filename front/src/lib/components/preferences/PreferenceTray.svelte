<script lang="ts">
	import { rUserPrefsOpen } from '$core/layout'
	import Icon from '../Icon.svelte'
	import Input from '../input/Input.svelte'
	import Preferences from './Preferences.svelte'

	let input: Input
	$: value = ''

	$: {
		if ($rUserPrefsOpen) input?.focus()
	}
</script>

<div class="tray" class:show={$rUserPrefsOpen}>
	<div class="tray-header">
		<Input
			bind:this={input}
			class="right-flat sm"
			placeholder="Search"
			inputClass={{ ty: 'StrClass', c: {} }}
			key="pref-search"
			bind:value
		/>
		<select class="sm left-flat">
			<option>Preferences</option>
			<option>Keymap</option>
			<option>Theme</option>
		</select>
		<button class="md"><Icon name="arrow-up" /></button>
	</div>
	<Preferences filter={value} />
</div>

<style>
	button {
		margin-right: var(--gap4);
	}
	.tray {
		--preferences-tray-width: 450px;
		position: relative;
		display: flex;
		flex-direction: column;
		position: absolute;
		width: var(--preferences-tray-width);
		min-width: max-content;
		max-height: calc(100vh - var(--md-nav) - var(--gap8));
		top: -10vh;
		right: 0px;
		border-radius: var(--pane-radius);
		transition: top var(--ui-speed) ease-out, opacity var(--ui-speed) linear;
		padding-inline: var(--gap4);
		padding-bottom: var(--gap4);
		z-index: var(--z-tray);
		opacity: 0;
		background-color: var(--border-primary);
		user-select: none;
		pointer-events: none;
	}
	.tray::after {
		content: '';
		position: absolute;
		right: 100%;
		top: -1px;
		width: calc(var(--pane-radius) * 2);
		height: calc(var(--pane-radius) * 2);
		border-top-right-radius: var(--pane-radius);
		box-shadow: 3px -2px 0 0 var(--border-primary);
	}
	.tray::after {
		content: '';
		position: absolute;
		left: calc(var(--pane-radius) * -2);
		top: -1px;
		width: calc(var(--pane-radius) * 2);
		height: calc(var(--pane-radius) * 2);
		border-top-right-radius: var(--pane-radius);
		box-shadow: 4px -2px 0 0 var(--border-primary);
	}
	.show {
		top: 0;
		opacity: 1;
		user-select: inherit;
		pointer-events: inherit;
	}
	.tray-header {
		height: var(--md-nav);
		min-height: var(--md-nav);
		display: flex;
		align-items: center;
	}
</style>
