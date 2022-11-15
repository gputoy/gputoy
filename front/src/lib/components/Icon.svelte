<script lang="ts">
	import feather from 'feather-icons'

	export let name: string
	export let width = '1.1em'
	export let height = '1.1em'
	export let rotation = '0deg'
	export let stroked = false
	$: icon = feather.icons[name]
</script>

{#if icon}
	<svg
		{...icon.attrs}
		style="width: {width};height: {height}; transform: rotate({rotation});"
		class:stroked
	>
		{@html icon.contents}
	</svg>
{/if}

<style>
	svg {
		overflow: visible;
		transform-origin: 50% 50%;
		fill: var(--text-accent-color);
		stroke: none;
	}
	.stroked {
		fill: none;
		stroke: var(--text-accent-color);
		stroke-width: 1.5px;
	}
	.stroked:hover {
		stroke: var(--text-color);
	}
</style>
