<script lang="ts">
	import feather, { type FeatherIconNames } from 'feather-icons'

	export let name: FeatherIconNames
	export let rotation = '0deg'
	export let stroke = false
	export let stroked = false
	export let thick = false
	export let filled = false
	export let style = ''
	$: icon = feather.icons[name] as any
</script>

{#if icon}
	<svg
		{...icon.attrs}
		style="transform: rotate({rotation});{style}"
		class:stroked
		class:stroke
		class:thick
		class:filled
		{...$$restProps}
	>
		{@html icon.contents}
	</svg>
{/if}

<style>
	svg {
		transform-origin: 50% 50%;
		transition: transform var(--transition-quick) ease-out;
	}
	/* svg:hover {
		fill: var(--text-important);
	} */
	.stroke {
		stroke-width: 2px;
	}
	.stroked {
		stroke-width: 2px;
	}
	.filled {
		fill: var(--text-accent-color);
	}
	.thick {
		stroke-width: 2.5px;
	}
</style>
