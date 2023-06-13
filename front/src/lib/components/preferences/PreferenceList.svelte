<script lang="ts">
	import { PREFERENCE_KEYS } from '$gen'
	import PreferenceController from './PreferenceController.svelte'

	export let filter: string | null

	export function focus() {}
</script>

<div class="body pane no-scrollbar">
	{#each PREFERENCE_KEYS as preferenceKey}
		<PreferenceController key={preferenceKey} {filter} />
	{/each}
</div>

<style>
	.body {
		flex: 1 1 auto;
		display: flex;
		flex-direction: column;
		gap: var(--gap8);
		overflow-y: scroll;
		max-width: 450px;
		min-width: 325px;
	}
</style>
