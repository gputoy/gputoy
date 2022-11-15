<script lang="ts">
	import Icon from '$lib/components/Icon.svelte'
	import { theme, toggle } from '$stores/theme'
	import IconButton from './IconButton.svelte'
</script>

<IconButton on:click={toggle}>
	{#if $theme == 'dark'}
		<Icon name="sun" stroked thick />
	{:else}
		<Icon name="moon" stroked thick />
	{/if}
</IconButton>
