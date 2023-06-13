<script lang="ts">
	import { preferenceDefault, resetPreference, wPref } from '$core/preferences'
	import { searchSplit } from '$core/util'
	import type { PreferenceKey, SchemaEntry } from '$gen'
	import schemas from '$gen/prefs/schema.json'
	import Input from '$lib/components/input/Input.svelte'
	import Select from '$lib/components/input/Select.svelte'
	import Toggle from '$lib/components/input/Toggle.svelte'
	import Icon from '../Icon.svelte'

	export let key: PreferenceKey
	export let filter: string | null
	let value = wPref(key)

	$: schema = schemas[key] as SchemaEntry
	$: isDefault = $value == preferenceDefault(key)
	$: path = key.slice(0, key.lastIndexOf('.'))
	$: parts = filter ? searchSplit(`${path}.${schema.name}`, filter) : null
	$: hide = filter && !parts

	function onHandleChange(event: CustomEvent<any>) {
		value.set(event.detail)
	}
</script>

<div class="root" class:hide>
	<div class="top">
		<div class="left">
			{#if parts}
				{parts[0]}
				<span class="highlighted">
					{parts[1]}
				</span>
				{parts[2]}
			{:else}
				{path}.{schema.name}
			{/if}
		</div>
		<div class="right">
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
	<p>{schema.description}</p>
	<div class="input">
		{#if schema.configClass.ty == 'EnumClass'}
			<Select
				class="sm"
				variants={schema.configClass.c.variants}
				bind:value={$value}
			/>
		{:else if schema.configClass.ty == 'BoolClass'}
			<Toggle class="sm" bind:value={$value} />
		{:else}
			<Input
				class="sm stretch"
				value={$value}
				inputClass={schema.configClass}
				{key}
				completionKey="Empty"
				last
				on:change={onHandleChange}
			/>
		{/if}
	</div>
</div>

<style>
	.root {
		background-color: var(--background-alt);
		border-radius: var(--pane-radius);
		box-sizing: border-box;
		min-width: min-content;
		width: 100%;
	}
	.top {
		padding: var(--gap4);
		padding-inline: var(--gap8);
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
	.left {
		font-size: var(--sm);
		font-family: var(--font-mono);
		color: var(--text-accent-color);
		flex: 0 0 auto;
		display: flex;
		align-items: center;
	}
	.right {
		display: flex;
		font-family: var(--font-mono);
		flex: 0 0 auto;
		display: flex;
		align-items: center;
		justify-content: right;
	}
	.input {
		padding: var(--gap4);
		width: 200px;
	}
	p {
		font-size: var(--xs);
		color: var(--text-accent-color);
		margin: var(--gap6);
	}
</style>
