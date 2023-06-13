<script lang="ts">
	import { page } from '$app/stores'
	import NavItem from '$lib/components/buttons/NavItem.svelte'
	import Logo from '$lib/components/Logo.svelte'
</script>

<header>
	<nav>
		<ul id="link-container">
			<div class="logo-container">
				<Logo />
				<NavItem
					title="GPUToy"
					current={$page.route.id === ''}
					style="font-weight: bold"
				/>
			</div>
			<NavItem
				title="Browse"
				route="/browse"
				current={$page.route.id === 'browse'}
			/>
			<NavItem title="Dev" route="/dev" current={$page.route.id === 'dev'} />
			<NavItem title="Docs" current={$page.route.id === 'docs'} />
		</ul>

		<div class="navend" />
	</nav>
</header>
<main>
	<slot />
</main>

<style>
	main {
		flex: 1 1 auto;
		width: 100%;
		height: 100%;
		flex-grow: 1;
		display: flex;
		flex-direction: column;
		align-items: center;
	}

	.navend {
		height: 100%;
		display: flex;
		justify-content: center;
		gap: 0.5rem;
		margin-right: 0.25rem;
		align-items: center;
	}
	.logo-container {
		display: flex;
		align-items: center;
		margin-right: 4rem;
	}

	header {
		z-index: 2;
		flex: 0 0 auto;
		height: 48px;
	}
	nav {
		position: relative;
		display: flex;
		flex-direction: row-reverse;
		height: 100%;
		background-color: var(--background-nav);
		justify-content: space-between;
		align-items: center;
		border-bottom: var(--border);
	}
	#link-container {
		position: absolute;
		left: 50%;
		translate: -50%;
		display: flex;
		flex-direction: row;
		gap: 0.5rem;
		margin: 0rem;
		margin-left: 0.25rem;
	}
</style>
