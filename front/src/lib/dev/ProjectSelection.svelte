<script lang="ts">
	import IconButton from '$lib/components/buttons/IconButton.svelte'
	import { initNewProject, loadAllProjects, setProject } from '$stores/project'
	import type { ProjectResponse } from 'src/generated/types'
	import Icon from 'svelte-awesome'
	import plus from 'svelte-awesome/icons/plus'
	let projectList = loadAllProjects()
	function onSetProject(project: ProjectResponse) {
		setProject(project, true)
	}
</script>

<div class="project-selection">
	<div class="projects-container">
		<div class="header">
			<h2>Projects</h2>
			<IconButton on:click={initNewProject} text="New project">
				<Icon data={plus} />
			</IconButton>
		</div>
		<div class="project-list">
			{#await projectList}
				Loading...
			{:then list}
				{#each list as project}
					<div class="project-item" on:click={() => onSetProject(project)}>
						<span class="project-title">{project.title}</span>
						<span class="project-date" />
					</div>
				{/each}
			{/await}
		</div>
	</div>
</div>

<style>
	.project-selection {
		display: flex;
		flex-direction: column;
		width: 100%;
		height: 100%;
		align-items: center;
		justify-content: center;
	}
	.projects-container {
		display: flex;
		flex-direction: column;
		width: 50%;
		min-height: 500px;
		border: var(--border);
	}
	.project-list {
		display: flex;
		flex: 1 1 auto;
		flex-direction: column;
		width: 100%;
	}
	.project-item {
		width: 100%;
		padding: 0.5rem;
		cursor: pointer;
	}

	.project-item:hover {
		background-color: var(--accent-color);
	}
	.header {
		height: 30px;
		padding: 1rem;
		display: flex;
		justify-content: space-between;
		align-items: center;
		flex: 0 0 auto;
		border-bottom: var(--border);
	}
</style>
