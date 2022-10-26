<script lang="ts">
	import { theme, toggle } from '$stores/theme'
	import Icon from 'svelte-awesome'
	import sunO from 'svelte-awesome/icons/moonO'
	import moonO from 'svelte-awesome/icons/sunO'
	import IconButton from './IconButton.svelte'
</script>

<IconButton on:click={toggle}>
	{#if $theme == 'dark'}
		<Icon data={sunO} />
	{:else}
		<Icon data={moonO} />
	{/if}
</IconButton>
