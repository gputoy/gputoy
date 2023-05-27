<script lang="ts">
	import { preferenceDefault, resetPreference, wPref } from '$core/preferences'
	import type { ConfigValueSchema, PreferenceKey } from '$gen'
	import Icon from '../Icon.svelte'
	import Input from '../input/Input.svelte'
	import Select from '../input/Select.svelte'
	import Toggle from '../input/Toggle.svelte'

	export let schema: ConfigValueSchema
	export let filter: string
	$: key = schema.path as PreferenceKey
	console.log(key)
	$: value = wPref(key)
	$: isDefault = $value == preferenceDefault(key)
	$: show = schema.path.includes(filter)
</script>

<div class="root" class:show>
	<div class="top">
		<div class="left">
			{schema.name}:
		</div>
		<div class="right">
			{#if schema.class.ty == 'EnumClass'}
				<Select {schema} bind:value={$value} />
			{:else if schema.class.ty == 'BoolClass'}
				<Toggle class="sm" bind:value={$value} />
			{:else}
				<Input
					class="sm stretch"
					bind:value={$value}
					inputClass={schema.class}
					key={schema.path}
					last
				/>
			{/if}
			<div class="buttons">
				<button class="sm clear right-flat" tabindex="-1">
					<Icon name="copy" stroked thick />
				</button>
				<button
					class="sm clear left-flat"
					tabindex="-1"
					on:click={() => resetPreference(key)}
					disabled={isDefault}
				>
					<Icon name="refresh-ccw" stroked thick />
				</button>
			</div>
		</div>
	</div>
	<p>{schema.description}</p>
</div>

<style>
	.root {
		background-color: var(--background-alt);
		border-radius: var(--pane-radius);
		box-sizing: border-box;
		min-width: min-content;
		width: 100%;
		display: none;
	}
	.show {
		display: block;
	}
	.top {
		height: var(--sm-nav);
		gap: var(--nav-gap);
		border-radius: var(--pane-radius);
		background-color: var(--background-nav);
		font-family: var(--font-mono);
		display: flex;
		justify-content: space-between;
		align-items: center;
		font-weight: bold;
	}
	.buttons {
		flex: 0 0 auto;
		display: flex;
		align-items: center;
	}
	.left {
		padding-left: var(--nav-gap);
		font-size: var(--sm);
		font-family: var(--font-mono);
		color: var(--text-accent-color);
		gap: var(--nav-gap);
		flex: 0 0 auto;
		display: flex;
		align-items: center;
	}
	.right {
		display: flex;
		min-width: 180px;
		font-family: var(--font-mono);
	}
	p {
		font-size: var(--xs);
		color: var(--text-accent-color);
		margin: var(--gap6);
	}
</style>
