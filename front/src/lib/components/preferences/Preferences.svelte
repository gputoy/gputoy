<script lang="ts">
	import type { ConfigValueSchema } from '$gen'
	import preferenceMetadata from '$gen/prefs/ui-tree.json'
	import Category from '$lib/components/preferences/Category.svelte'

	export let filter: string

	const categories = Object.values(
		(preferenceMetadata as unknown as ConfigValueSchema).class.c
	)
</script>

<div class="body pane no-scrollbar">
	{#each categories as category}
		<Category schema={category} {filter} />
	{/each}
</div>

<style>
	.body {
		flex: 1 1 auto;
		display: flex;
		flex-direction: column;
		gap: var(--gap2);
		overflow-y: scroll;
		max-width: 450px;
		min-width: 325px;
	}
</style>
