<script lang="ts">
	import { toggleTheme } from '$core/util'
	import IconButton from '$lib/components/buttons/IconButton.svelte'
	import Icon from '$lib/components/Icon.svelte'
	import { wTheme } from '$stores'
</script>

<IconButton on:click={toggleTheme}>
	{#if $wTheme == 'dark'}
		<Icon name="sun" stroked thick />
	{:else}
		<Icon name="moon" stroked thick />
	{/if}
</IconButton>
