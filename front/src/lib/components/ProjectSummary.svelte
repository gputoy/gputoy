<script lang="ts">
	import { wProjectMeta } from '$stores'
</script>

<div id="root">
	<h2 class="title">{$wProjectMeta.title}</h2>
	<p class="desc">
		{$wProjectMeta.description}
	</p>
</div>

<style>
	#root {
		min-width: max-content;
		padding: 0.5rem;
		padding-left: 1rem;
	}
	.title {
		margin-block: 0rem;
		font-size: var(--lg);
	}
	.desc {
		font-size: var(--xs);
	}
</style>
