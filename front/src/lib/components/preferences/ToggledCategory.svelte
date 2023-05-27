<script lang="ts">
	import type { CategoryClass, ConfigValueSchema } from '$gen'
	import PreferenceEntry from './PreferenceEntry.svelte'

	export let schema: ConfigValueSchema
	$: children = schema.class.c as CategoryClass
</script>

<div class="root">
	<!-- <PreferenceController {metadata} description={metadata.description}>
		<input type="checkbox" />
	</ControllerHeader> -->
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
