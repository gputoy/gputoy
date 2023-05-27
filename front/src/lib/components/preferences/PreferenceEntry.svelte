<script lang="ts">
	import type { ConfigValueSchema } from '$gen'
	import Category from './Category.svelte'
	import PreferenceController from './PreferenceController.svelte'
	import ToggledCategory from './ToggledCategory.svelte'

	export let filter: string = ''
	export let schema: ConfigValueSchema
	$: ty = schema.class.ty
</script>

{#if ty == 'CategoryClass'}
	<Category {schema} {filter} />
{:else if ty == 'ToggledCategoryClass'}
	<ToggledCategory {schema} />
{:else}
	<PreferenceController {schema} {filter} />
{/if}
