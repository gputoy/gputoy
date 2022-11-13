<script lang="ts">
	import type { SupportedExtension } from 'src/generated/types'

	export let size = 32
	export let extension: SupportedExtension

	function colorFromType(ext: SupportedExtension): string {
		switch (ext) {
			case 'glsl':
				return '#f3722c'
			case 'wgsl':
				return '#277da1'
			case 'json':
				return '#90be6d'
			case 'txt':
				return '#495057'
			default:
				return '#4d908e'
		}
	}
</script>

<svg
	width={size}
	height={size}
	version="1.1"
	viewBox="0 0 16.933 16.933"
	xmlns="http://www.w3.org/2000/svg"
	{...$$restProps}
>
	<path
		class="path0"
		d="m4.6982 0.26458 5.8851-5e-7 4.7625 4.7625-1e-6 10.118c0 0.84386-0.67935 1.5232-1.5232 1.5232h-9.1244c-0.84386 0-1.5232-0.67935-1.5232-1.5232v-13.358c0-0.84386 0.67935-1.5232 1.5232-1.5232z"
		fill={colorFromType(extension)}
	/>
	<path class="path1" d="m10.583 5.0271v-4.7625l4.7625 4.7625z" fill="#1a1a1a" />
	<rect x="2" y="8" width="13" height="8" rx="0" ry="1" />
	<text>
		<tspan x="8" y="14">
			{extension}
		</tspan>
	</text>
</svg>

<style>
	.path0 {
		stroke-width: 0;
	}
	.path1 {
		fill-opacity: 0.5;
		stroke-opacity: 0;
	}
	rect {
		fill: var(--background-alt);
		stroke-width: 0;
	}
	tspan {
		fill: var(--text-important);
		font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
		font-size: 7px;
		text-anchor: middle;
		stroke-width: '.3';
	}
</style>
