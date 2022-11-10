<script lang="ts">
	export let title: string
	export let route: string = '/'
	export let current: boolean = false
	export let aClass: string = ''
</script>

<div class={current ? 'current' : ''} {...$$restProps}>
	<a href={route ?? '/'} class={aClass}>
		{title}
	</a>
</div>

<style>
	div {
		/* font-family: var(--font-body); */
		font-size: var(--md);
		height: 100%;
		cursor: pointer;
		transition: background-color 0.1s ease;
	}

	a:hover {
		color: inherit;
	}

	a {
		color: var(--text-accent-color);
		text-decoration: none;
		width: 100%;
		padding: 3px 5px;
		border-radius: 4px;
	}
	.current > a {
		/* background-color: var(--background-nav); */
		background-color: var(--glass-low);
		outline: 1px solid var(--border-secondary);
		color: var(--accent-color);

		font-weight: black;
	}

	.current:hover {
		background-color: inherit;
	}
</style>
