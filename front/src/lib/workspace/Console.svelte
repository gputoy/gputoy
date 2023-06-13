<script lang="ts">
	import {
		LOG_PREFIX,
		LOG_PREFIX_STYLES,
		submitCommand,
		toLogLevel
	} from '$core/console'
	import { rTerminalOpen } from '$core/layout'
	import { rPref } from '$core/preferences'
	import Input from '$lib/components/input/Input.svelte'
	import { wConsole } from '$stores'

	let logsEl: HTMLDivElement
	let value = ''
	let input: Input

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
	function handleSubmit(event: CustomEvent<string>) {
		submitCommand(event.detail)
		input.clear()
		setTimeout(() => {
			logsEl.scrollTo({ behavior: 'auto', top: logsEl.scrollHeight })
		}, 10)
	}
</script>

<div class="container" class:show={$rTerminalOpen}>
	<div class="prompt-container" class:wrap={$consoleWrap}>
		<div class="prompt-line">
			<span style="user-select: none;" class="prefix">~</span>
			<Input
				bind:this={input}
				bind:value
				key="cmd"
				class="med"
				on:submit={handleSubmit}
				completionKey={null}
				inputClass={null}
			/>
		</div>
	</div>
	<div class="log-container" bind:this={logsEl}>
		{#each filteredConsole as log}
			<span class="log">
				<span class="prefix" style={LOG_PREFIX_STYLES.get(log.level)}
					>{LOG_PREFIX.get(log.level)}</span
				>
				{@html log.message}
				<!-- {@html '<code style="background-color:var(--accent-color-transparent);">here is code</code>'} -->
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
</style>
