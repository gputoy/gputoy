<script lang="ts">
	import type { CategoryClass, ConfigValueSchema } from '$gen'
	import PreferenceEntry from './PreferenceEntry.svelte'

	export let schema: ConfigValueSchema
	$: children = schema.class.c as CategoryClass

	$: console.log('schema: ', schema)

	// const checkboxSchema: ConfigValueSchema = {
	// 	class: 'BoolClass',
	// 	description: 'Enable ' + schema.name,
	// 	name: schema.name,
	// 	path: schema.path + '.enabled'
	// }
</script>

<div class="root">
	<!-- <PreferenceEntry schema={{ class: '' }} /> -->

	<ul>
		{#each Object.values(children) as child}
			<PreferenceEntry schema={child} />
		{/each}
	</ul>
</div>

<style>
	ul {
		margin: 0px;
		padding-left: 1rem;
	}
	div {
		display: flex;
		flex-direction: column;
	}
</style>
