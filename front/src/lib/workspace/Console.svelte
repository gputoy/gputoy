<script lang="ts">
	import { LOG_PREFIX, LOG_PREFIX_STYLES, toLogLevel } from '$core/console'
	import { rTerminalOpen } from '$core/layout'
	import { rPref } from '$core/preferences'
	import type { ConfigValueClass } from '$gen'
	import Input from '$lib/components/input/Input.svelte'
	import { wConsole } from '$stores'

	let value = ''
	let input: Input
	$: inputClass = {
		ty: 'CmdClass',
		c: {
			completions: $showCompletions
		}
	} as ConfigValueClass

	let showCompletions = rPref('console.show-completions')
	let consoleLevel = rPref('console.level')
	let consoleWrap = rPref('console.wrap')

	$: if ($rTerminalOpen) setTimeout(() => input.focus(), 1)
	$: filteredConsole = $wConsole.filter((log) => {
		return log.level >= toLogLevel($consoleLevel)
	})

	// function handleCommand(event: CustomEvent<string>) {
	// 	const consoleCommand = event.detail
	// 	wConsole.echo(consoleCommand)
	// 	input.clear()
	// 	let result = action(consoleCommand)
	// 	if ('message' in result) {
	// 		handleClientError(result)
	// 	} else {
	// 		pushAction(result)
	// 	}
	// }
</script>

<div class="container" class:show={$rTerminalOpen}>
	<div class="prompt-container" class:wrap={$consoleWrap}>
		<div class="prompt-line">
			<span style="user-select: none;" class="prefix">~</span>
			<Input bind:this={input} bind:value {inputClass} key="cmd" class="med" />
		</div>
	</div>
	<div class="log-container">
		{#each filteredConsole as log}
			<span class="log">
				<span class="prefix" style={LOG_PREFIX_STYLES.get(log.level)}
					>{LOG_PREFIX.get(log.level)}</span
				>
				{log.message}
			</span>
		{/each}
	</div>
</div>

<style>
	.container {
		visibility: hidden;
		--padding: 8px;
		position: absolute;
		left: 0px;
		bottom: 0px;
		width: 100%;
		height: fit-content;
		min-height: 200px;
		max-height: 100%;
		display: flex;
		flex-direction: column-reverse;
		font-family: var(--font-mono);
		font-size: var(--sm);
	}

	.prompt-container {
		position: relative;
		flex-direction: row;
		align-items: center;
		padding-inline: var(--padding);
		/* background-color: var(--glass-med); */
		border-radius: var(--pane-radius);
		height: 100%;
		font-size: var(--sm);
		/* padding-block: 2px; */
	}
	.prompt-line {
		display: flex;
		align-items: center;
	}
	.prompt-line span {
		flex: 0 0 auto;
		margin-right: var(--gap2);
	}

	.log-container {
		display: flex;
		flex-direction: column;
		overflow-y: scroll;
	}
	.log {
		padding-inline: var(--padding);
	}
	.show {
		visibility: visible;
	}
	.prefix {
		font-weight: bold;
		font-size: var(--xs);
	}

	.prompt-container:global(*) {
		background-color: red !important;
	}
</style>
