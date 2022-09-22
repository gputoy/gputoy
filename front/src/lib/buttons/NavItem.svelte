<script lang="ts">
	export let title: string;
	export let route: string = '/';
	export let current: boolean = false;
	export let aClass: string = '';
</script>

<div class={current ? 'current' : ''}>
	<a href={route ?? '/'} class={aClass}>
		{title}
	</a>
</div>

<style>
	div {
		/* font-family: var(--font-body); */
		font-size: var(--sm);
		height: 100%;
		cursor: pointer;
		transition: background-color 0.1s ease;
	}

	a:hover {
		color: var(--text-accent-color);
	}

	a {
		color: inherit;
		text-decoration: none;
		width: 100%;
		padding: 3px;
		border-radius: 4px;
	}

	.current {
		border-color: var(--accent-color);
	}

	.current > a {
		background-color: var(--secondary-color);
		border: 1px solid var(--border-secondary);
	}

	.current:hover {
		background-color: inherit;
	}
</style>
