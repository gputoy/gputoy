<script lang="ts">
	import { theme, toggle } from '$stores/theme'
	import Icon from 'svelte-awesome'
	import lightBulbO from 'svelte-awesome/icons/lightbulbO'
	import moonO from 'svelte-awesome/icons/moonO'
	import IconButton from './IconButton.svelte'
</script>

<IconButton on:click={toggle}>
	{#if $theme == 'dark'}
		<Icon data={lightBulbO} />
	{:else}
		<Icon data={moonO} />
	{/if}
</IconButton>
