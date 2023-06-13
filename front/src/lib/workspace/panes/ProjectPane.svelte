<script lang="ts">
	import Accordian from '$lib/components/Accordian.svelte'
	import FileNode from '$lib/components/file/FileNode.svelte'
	import ProjectSummary from '$lib/workspace/ProjectSummary.svelte'
	import { dFileTree } from '$stores'
</script>

<div id="pane-root">
	<Accordian title="Summary">
		<svelte:fragment slot="menu" />
		<svelte:fragment slot="content">
			<ProjectSummary />
		</svelte:fragment>
	</Accordian>
	<Accordian title="Files">
		<svelte:fragment slot="menu" />
		<svelte:fragment slot="content">
			{#if $dFileTree}
				<FileNode fileNode={$dFileTree} />
			{/if}
		</svelte:fragment>
	</Accordian>
	<Accordian title="Runners">
		<svelte:fragment slot="menu" />
		<svelte:fragment slot="content" />
	</Accordian>
	<Accordian title="Resources">
		<svelte:fragment slot="menu" />
		<svelte:fragment slot="content" />
	</Accordian>
	<!-- <Accordian title="System">
		<svelte:fragment slot="content">
			<div>
				<label>Client: {getContextHealth()}</label>
			</div>
			<div>
				<label>Analyzer: {getAnalyzerHealth()}</label>
			</div>
		</svelte:fragment>
	</Accordian> -->
	<div class="filler" />
</div>

<style>
	#pane-root {
		display: flex;
		flex-direction: column;
		width: 100%;
		height: 100%;
		border-radius: var(--pane-radius);
		box-sizing: border-box;
		margin: 0;
		gap: 2px;
		border-right: var(--border);
	}
	.filler {
		border-radius: var(--pane-radius);
		flex: 1 1 auto;
		background-color: var(--background-alt);
	}
</style>
