<script lang="ts">
	import Accordian from '$lib/components/Accordian.svelte'
	import IconButton from '$lib/components/buttons/IconButton.svelte'
	import FileNode from '$lib/components/file/FileNode.svelte'
	import Icon from '$lib/components/Icon.svelte'
	import ProjectSummary from '$lib/components/ProjectSummary.svelte'
	import { dFileTree } from '$stores'
</script>

<div id="pane-root">
	<Accordian title="Summary">
		<svelte:fragment slot="menu">
			<IconButton empty>
				<Icon stroked name="edit-2" />
			</IconButton>
		</svelte:fragment>
		<svelte:fragment slot="content">
			<ProjectSummary />
		</svelte:fragment>
	</Accordian>
	<Accordian title="Files">
		<svelte:fragment slot="menu">
			<IconButton empty title="New file">
				<Icon stroked name="file-plus" />
			</IconButton>
			<IconButton empty title="New directory">
				<Icon stroked name="folder-plus" />
			</IconButton>
		</svelte:fragment>
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
</div>

<style>
	#pane-root {
		display: flex;
		flex-direction: column;
		width: 100%;
		height: 100%;
		background-color: var(--background-alt);
		margin: 0;
	}
</style>
