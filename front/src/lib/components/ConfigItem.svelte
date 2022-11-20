<script lang="ts">
	import {
		USER_CONFIG_META,
		type ConfigItemMeta,
		type ConfigKey,
		type ConfigScope
	} from '$core/consts'
	import { setProperty, validate } from '$core/preferences'
	export let value: any
	export let scope: ConfigScope
	export let key: ConfigKey

	//@ts-ignore
	let configMeta = USER_CONFIG_META[scope][key] as ConfigItemMeta

	function onChangeProp(ev: Event) {
		const value = (ev.target as HTMLInputElement)?.value
		if (validate(scope, key, value)) setProperty(scope, key, value)
	}
	function onToggleProp(ev: Event) {
		const value = (ev.target as HTMLInputElement)?.value
		setProperty(scope, key, value == 'on' ? true : false)
	}
	function onSelectProp(ev: Event) {
		const value = (ev.target as HTMLSelectElement)?.value
		if (validate(scope, key, value)) setProperty(scope, key, value)
	}
	// TODO: move to utils as this will be needed later in console
	function toKebabCase(key: string) {
		return key
			.split('')
			.map((char) => {
				if (char.toUpperCase() == char) return '-' + char.toLowerCase()
				return char
			})
			.join('')
	}

	$: kebabCaseKey = toKebabCase(key)
</script>

<div>
	<label for={key}>
		<code>
			{kebabCaseKey}
		</code>
	</label>
	<p>
		{configMeta.description}
	</p>
	<div class="input-wrapper">
		{#if configMeta.type === 'select'}
			<select id={key} on:change={onSelectProp}>
				{#each configMeta.options ?? [] as value}
					<option {value}>{value}</option>
				{/each}
			</select>
		{:else if configMeta.type === 'toggle'}
			<input type="checkbox" on:change={onToggleProp} checked={value} />
		{:else}
			<input
				id={key}
				type={configMeta.type}
				{value}
				on:change={onChangeProp}
				min={configMeta.min}
				max={configMeta.max}
			/>
			<span class="unit">{configMeta.units ?? ''} </span>
		{/if}
	</div>
</div>

<style>
	.input-wrapper {
		position: relative;
	}
	.unit {
		color: var(--text-accent-color);
		font-size: var(--xs);
		position: absolute;
	}
	label {
		font-size: var(--md);
	}
	input {
		padding: 4px;
		outline: var(--border2);
		font-size: var(--xs);
	}
	p {
		margin-block: 4px;
		color: var(--text-accent-color);
		font-size: var(--xs);
	}
	input::-webkit-outer-spin-button,
	input::-webkit-inner-spin-button {
		-webkit-appearance: none;
		margin: 0;
	}

	input[type='number'] {
		-moz-appearance: textfield;
	}
</style>
