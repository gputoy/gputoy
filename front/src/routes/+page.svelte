<script lang="ts">
	import { OnMount } from 'fractils';
	import { fade } from 'svelte/transition';
</script>

<svelte:head>
	<title>gputoy</title>
	<meta name="description" content="Gputoy" />
</svelte:head>

<section>
	<div class="hero-container">
		<OnMount>
			<div class="header-container">
				<h1 in:fade={{ delay: 500, duration: 500 }}>All those gigaflops, sitting idle</h1>
				<h1 in:fade={{ delay: 1750, duration: 500 }}>Put them to work</h1>
			</div>
			<p>
				Lorem ipsum dolor sit, amet consectetur adipisicing elit. Repudiandae nobis officiis
				sapiente inventore voluptate a doloribus blanditiis distinctio ab laborum pariatur possimus,
				odio commodi exercitationem sunt reiciendis. Suscipit, tempora unde!
			</p>
			<!-- <h1>All those gigaflops, sitting idle</h1> -->
		</OnMount>
	</div>
</section>
<section>
	<!-- <h1 transition:fade|local={{ delay: 500, duration: 1000 }}>Here is more text to look at</h1> -->
</section>
<section>
	<!-- <h1 transition:fade|local={{ delay: 500, duration: 1000 }}>Here is more text to look at</h1> -->
</section>

<style>
	section {
		flex: 0 0 auto;
		height: 100%;
		width: 50%;
		display: flex;
	}

	.hero-container {
		display: flex;
		justify-content: center;
		align-items: center;
		flex-direction: column;
	}

	.header-container {
		display: flex;
		justify-content: center;
		align-items: left;
		flex-direction: column;
	}

	h1 {
		line-height: 0%;
	}

	p {
		position: relative;
		left: 10%;
		width: 60%;
		text-align: end;
		font-size: var(--xl);
		color: var(--text-accent-color);
	}
</style>
