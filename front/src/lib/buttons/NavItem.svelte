<script lang="ts">
	export let title: string;
	export let route: string = '/';
	export let current: boolean = false;
	export let aClass: string = '';
</script>

<div class={current ? 'current' : ''}>
	<div class="pad" />
	<a href={route ?? '/'} class={aClass}>
		{title}
	</a>
</div>

<style>
	.pad {
		height: 5px;
		width: 100%;
		border: none;
	}
	div {
		display: flex;
		flex-direction: column;
		align-items: center;
		text-align: center;
		/* font-family: var(--font-body); */
		font-size: var(--sm);
		border-bottom: 1px solid var(--text-color);
		width: fit-content;
		height: 100%;
		cursor: pointer;
		transition: background-color 0.1s ease;
	}

	a:hover {
		background-color: var(--glass-med);
	}

	a {
		align-self: center;
		color: inherit;
		text-decoration: none;
		width: 100%;
		padding: 0.3rem;
		border-radius: 4px;
	}

	.current {
		border-color: var(--accent-color);
	}

	.current > a {
		font-weight: 600;
		background-color: var(--secondary-color);
	}

	.current:hover {
		background-color: inherit;
	}
</style>
