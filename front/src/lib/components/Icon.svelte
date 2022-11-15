<script lang="ts">
	import feather from 'feather-icons'

	export let name: string
	export let size = '1.1em'
	export let rotation = '0deg'
	export let stroke = false
	export let stroked = false
	export let thick = false
	export let style = ''
	$: icon = feather.icons[name]
</script>

{#if icon}
	<svg
		{...icon.attrs}
		style="width: {size};height: {size}; transform: rotate({rotation});{style}"
		class:stroked
		class:stroke
		class:thick
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
	svg:hover {
		fill: var(--text-important);
	}
	.stroke {
		stroke: var(--text-accent-color);
		stroke-width: 1.5px;
	}
	.stroke:hover {
		stroke: var(--text-important);
	}
	.stroked {
		fill: none;
		stroke: var(--text-accent-color);
		stroke-width: 1.5px;
	}
	.stroked:hover {
		stroke: var(--text-important);
		fill: none;
	}
	.thick {
		stroke-width: 2.5px;
	}
</style>
