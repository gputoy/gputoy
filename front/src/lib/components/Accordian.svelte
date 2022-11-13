<script lang="ts">
	import { wLayout } from '$stores/project'
	export let title: string
	function handleClick() {
		wLayout.toggleAccordian(title)
	}
</script>

<div class="accordian-container">
	<div class="accordian-title" on:click={handleClick}>
		{title}
	</div>
	{#if $wLayout.accordianOpen[title] ?? false}
		<div class="accordian-content">
			<slot />
		</div>
	{/if}
</div>

<style>
	.accordian-container {
		display: flex;
		height: fit-content;
		flex-direction: column;
	}
	.accordian-title {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding-inline: 1rem;
		flex: 0 0 auto;
		height: 32px;
		background-color: var(--background-alt);
		border-bottom: 1px solid var(--border-secondary);
		user-select: none;
	}
	.accordian-title:hover {
		background-color: var(--glass-med);
	}
	.accordian-content {
		flex: 1 1 auto;
		border-bottom: 1px solid var(--border-secondary);
	}
</style>
