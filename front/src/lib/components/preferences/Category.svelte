<script lang="ts">
	import type { CategoryClass, ConfigValueSchema } from '$gen'
	import Accordian from '../Accordian.svelte'
	import PreferenceEntry from './PreferenceEntry.svelte'

	export let schema: ConfigValueSchema
	export let filter: string
	$: children = schema.class.c as CategoryClass
	$: values = Object.values(children)
	$: hide = values.length == 0
</script>

<Accordian title={schema.name}>
	<svelte:fragment slot="content">
		<ul>
			{#each values as schema}
				<PreferenceEntry {filter} {schema} />
			{/each}
		</ul>
	</svelte:fragment>
</Accordian>

<style>
	ul {
		display: flex;
		flex-direction: column;
		gap: var(--gap8);
		margin: 0px;
		width: 100%;
		margin: var(--gap4);
	}
</style>
