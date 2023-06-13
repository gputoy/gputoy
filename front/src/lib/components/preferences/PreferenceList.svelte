<script lang="ts">
	import { PREFERENCE_KEYS } from '$gen'
	import PreferenceController from './PreferenceController.svelte'

	export let filter: string | null
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
		gap: var(--gap4);
		overflow-y: scroll;
		max-width: 450px;
		min-width: 325px;
	}
</style>
