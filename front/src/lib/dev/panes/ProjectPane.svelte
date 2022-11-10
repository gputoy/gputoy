<script lang="ts">
	import { dCanModifyProject, wProjectMeta } from '$stores/project'
</script>

<div id="pane-root">
	{#if $dCanModifyProject}
		<input class="title clear" type="text" bind:value={$wProjectMeta.title} />
		<textarea class="desc clear" type="text" bind:value={$wProjectMeta.description} />
	{:else}
		<h1 class="title">{$wProjectMeta.title}</h1>
	{/if}
</div>

<style>
	#pane-root {
		display: flex;
		flex-direction: column;
		width: 100%;
		height: 100%;
		background-color: var(--background-content);
		margin: 0;
		border: 1px transparent solid;
	}

	.title {
		font-size: var(--xl);
		margin: 8px;
		padding: 8px;
	}

	.clear {
		background-color: transparent;
		margin-left: 4px;
	}

	textarea {
		align-self: center;
	}
</style>
