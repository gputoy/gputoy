<script>
	import { page } from '$app/stores'
</script>

<div>
	<p>Oops!</p>
	<h1>{$page.status}: {$page.error?.message ?? 'Unknown error'}</h1>
</div>

<style>
	h1 {
		font-size: 3em;
	}
	div {
		width: 100%;
		height: 100%;
		display: flex;
		align-items: center;
		justify-content: center;
		flex-direction: column;
	}
</style>
