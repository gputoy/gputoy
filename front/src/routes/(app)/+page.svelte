<script lang="ts">
	import { OnMount } from 'fractils'
</script>

<svelte:head>
	<title>gputoy</title>
	<meta name="description" content="Gputoy" />
</svelte:head>

<section>
	<div class="hero-container">
		<OnMount />
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
		--bg-size: 10px;
		flex: 0 0 auto;
		flex-direction: column;
		height: 100%;
		width: 100%;
		display: flex;
		align-items: center;
		justify-content: center;
		background-image: radial-gradient(var(--matrix-color) 20%, transparent 20%);
		background-position: 0 0, var(--bg-size) var(--bg-size);
		background-size: var(--bg-size) var(--bg-size);
	}
	section::after {
		content: '';
		--bg-size: 100%;
		position: absolute;
		left: -50%;
		right: -50%;
		top: -100%;
		bottom: -100%;
		background-position: 0 0, var(--bg-size) var(--bg-size);
		background-size: calc(var(--bg-size) * 2);
		background-image: linear-gradient(
			152deg,
			var(--background-content) 0%,
			var(--accent-color) 50%,
			var(--accent-color) 55%,
			var(--background-content) 100%
		);
		animation: gradient 30s ease infinite;
		mix-blend-mode: var(--blend-mode);
		backdrop-filter: blur(16px);
		filter: blur(16px);
	}

	@keyframes gradient {
		0% {
			transform: rotate(0deg);
			background-position: 15% 0%;
		}
		50% {
			background-position: 86% 100%;
		}
		100% {
			/* transform: rotate(360deg); */
			background-position: 15% 0%;
		}
	}

	.hero-container {
		width: 50%;
		display: flex;
		justify-content: center;
		align-items: center;
		flex-direction: column;
		z-index: 1;
	}
	.hero-buttons {
		display: flex;
		flex-direction: row;
		gap: 2rem;
	}

	.header-container {
		display: flex;
		justify-content: center;
		align-items: left;
		flex-direction: column;
	}

	h1 {
		margin-block: 10px;
	}

	p {
		position: relative;
		left: 10%;
		width: 60%;
		text-align: end;
		font-size: var(--xl);
		color: var(--text-accent-color);
		margin-bottom: 2rem;
	}
</style>
