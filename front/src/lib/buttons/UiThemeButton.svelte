<script lang="ts">
	import { theme, toggle } from '$stores/theme';
	import FaRegMoon from 'svelte-icons/fa/FaRegMoon.svelte';
	import FaRegSun from 'svelte-icons/fa/FaRegSun.svelte';
	import IconButton from './IconButton.svelte';
</script>

<IconButton on:click={toggle}>
	{#if $theme == 'dark'}
		<FaRegSun />
	{:else}
		<FaRegMoon />
	{/if}
</IconButton>
