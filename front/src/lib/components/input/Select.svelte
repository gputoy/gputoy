<script lang="ts">
	import type { ConfigValueSchema, EnumClass } from '$gen'
	export let value: any
	export let schema: ConfigValueSchema

	function handleChange(event: Event) {
		console.log('change', event)
	}

	$: variants = (schema.class.c as EnumClass).variants
</script>

<select bind:value name={schema.name} {...$$restProps} on:change={handleChange}>
	{#each variants as variant}
		<option>{variant}</option>
	{/each}
</select>

<style>
</style>
