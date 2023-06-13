<script lang="ts">
	export let value: any
	export let variants: string[]

	function handleChange(event: Event) {
		console.log('change', event)
	}
</script>

<select bind:value {...$$restProps} on:change={handleChange}>
	{#each variants as variant}
		<option>{variant}</option>
	{/each}
</select>

<style>
	select {
		width: 100%;
	}
</style>
