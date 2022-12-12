<script lang="ts">
	import {
		generateCompletions,
		LOG_PREFIX,
		LOG_PREFIX_STYLES,
		submitConsoleComand
	} from '$core/console'
	import { wConsole, wConsoleOpen } from '$stores'

	let promptText = ''
	let input: HTMLInputElement
	$: completions = generateCompletions(promptText)
	function handleCommandSubmit(ev: KeyboardEvent) {
		if (ev.key === 'Enter') {
			submitConsoleComand(promptText)
			promptText = ''
		}
	}
	function handleKeypress(event: KeyboardEvent) {
		console.log(event)
		if (event.key == 'Tab') {
			console.log('Tab pressed')
			event.preventDefault()
		}
	}

	$: {
		if ($wConsoleOpen) setTimeout(() => input.focus(), 5)
	}
</script>

<div class="container" class:show={$wConsoleOpen}>
	<div class="prompt-container">
		<div class="prompt-line">
			<span style="user-select: none;"> ~ </span>
			<input
				bind:this={input}
				bind:value={promptText}
				on:keypress|capture={handleKeypress}
				tabindex="0"
				on:keydown={handleCommandSubmit}
			/>
		</div>
		{#if completions.length > 0}
			<div class="completions-container">
				{#each completions as completion}
					<div class="completion">
						<span>{completion.action}</span>
					</div>
				{/each}
			</div>
		{/if}
	</div>
	<div class="log-container">
		{#each $wConsole as log}
			<span class="log">
				<span style={LOG_PREFIX_STYLES.get(log.level)}>{LOG_PREFIX.get(log.level)}</span>
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
		max-height: 100%;
		display: flex;
		flex-direction: column-reverse;
		font-family: var(--font-mono);
		font-size: var(--sm);
		overflow-y: scroll;
	}
	.prompt-container {
		position: relative;
		flex-direction: row;
		background-color: var(--transparent-med);
		padding-inline: var(--padding);
	}
	.prompt-line {
		display: flex;
		align-items: center;
	}
	.prompt-line span {
		flex: 0 0 auto;
		font-weight: bold;
	}
	.prompt-line input {
		flex: 1 1 auto;
		background-color: transparent;
		outline: none;
		font-family: var(--font-mono);
		font-size: var(--sm);
	}
	.completions-container {
		position: absolute;
		left: 8px;
		width: fit-content;
		bottom: 100%;
		background-color: var(--background-alt);
		padding-block: 8px;
		border: var(--border);
		color: var(--text-accent-color);
	}
	.completion {
		display: flex;
		justify-content: space-between;
		padding-inline: var(--padding);
		font-size: var(--xs);
	}
	.log-container {
		display: flex;
		flex-direction: column;
	}
	.log {
		padding-inline: var(--padding);
		white-space: pre-wrap;
	}
	.show {
		visibility: visible;
	}
</style>
