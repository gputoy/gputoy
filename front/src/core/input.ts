import { pushAction } from '$core/actions'
import type { Completions } from '$core/completions'
import * as completions from '$core/completions'
import type { ConfigValueClass } from '$gen'
import { wConsole } from '$stores'
import * as wasm from '$wasm/common'
import { writable, type Readable, type Writable } from 'svelte/store'
import { handleClientError } from './console'

type UserPositonState = {
	activeWord: string
	wordIndex: number
	wordStart: number
	wordEnd: number
}

type InputRef = {
	key: string
	elem: HTMLInputElement
	class: ConfigValueClass
	onChange: (val: any) => void
}

class Input {
	private history: string[] = []
	private historyIdx: number = -1
	private stashedInput: string | null = null

	constructor(
		private ref: InputRef,
		private wCompletions: Writable<Completions>
	) {}

	elem(): HTMLInputElement {
		return this.ref.elem
	}

	attach() {
		const elem = this.elem()
		elem.addEventListener('keydown', this.onKeyDown.bind(this))
		elem.addEventListener('keypress', this.onKeyPress.bind(this))
	}

	deattach() {
		const elem = this.elem()
		elem.removeEventListener('keydown', this.onKeyDown)
		elem.removeEventListener('keypress', this.onKeyDown)
	}

	private onKeyDown(event: KeyboardEvent) {
		const elem = this.elem()
		if (!elem) return
		switch (event.key) {
			case 'Esc':
				console.log('Esc')
				break
			case 'Tab':
				let userPosition = this.getUserPosition(elem)
				if (userPosition.activeWord.length == 0) {
					this.refreshCompletions(true)
					this.moveCompletionIndex(0, userPosition)
				} else if (event.shiftKey) this.moveCompletionIndex(-1, userPosition)
				else this.moveCompletionIndex(1, userPosition)
				event.preventDefault()
				break
			case 'ArrowUp':
				this.moveHistoryIndex(1)
				event.preventDefault()
				break
			case 'ArrowDown':
				this.moveHistoryIndex(-1)
				event.preventDefault()
				break
			case 'ArrowLeft':
			case 'ArrowRight':
				this.refreshCompletions()
				break
			case 'Enter':
				this.ref.onChange(this.ref.elem.value)
				event.preventDefault()
				break
			case 'Backspace':
			case 'Delete':
				let start = elem.selectionStart || 0
				let end = elem.selectionEnd || 0

				if (start < end) start++

				this.removeRange(start, end)
				elem.setSelectionRange(start - 1, start - 1)
				this.refreshCompletions()

				event.preventDefault()
				break
		}
	}

	private onKeyPress(event: KeyboardEvent) {
		if (!this.elem) return
		switch (event.key) {
			case 'Tab':
			case 'ArrowUp':
			case 'ArrowDown':
			case 'Enter':
			case 'Backspace':
			case 'Delete':
				event.preventDefault()
				break
			default:
				let charCode = event.charCode || event.which
				this.pushChar(String.fromCharCode(charCode))
				this.refreshCompletions()
				event.preventDefault()
				event.stopImmediatePropagation()
		}
	}

	moveCompletionIndex(shift: number, userPosition: UserPositonState) {
		// const elem = this.elem()
		// let completion: completions.Completion | undefined
		// let currentIndex: number
		// this.wCompletions.update((completions) => {
		//     let newIndex = completions.index + shift
		//     if (newIndex < 0) newIndex = Math.max(-1, newIndex + completions.len)
		//     completions.index = newIndex % completions.len
		//     currentIndex = completions.index
		//     completion = completions.list[completions.index]
		//     return completions
		// })
		// if (completion) {
		//     let insertText = completions.getCompletionInsertText(completion)
		//     let start = elem.value.substring(0, userPosition.wordStart)
		//     let end = elem.value.substring(userPosition.wordEnd)
		//     elem.value = start.concat(insertText, end)
		// }
	}

	moveHistoryIndex(shift: number) {
		const elem = this.elem()
		if (!elem) return
		let oldIndex = this.historyIdx
		let newIndex = Math.min(
			this.history.length - 1,
			Math.max(-1, this.historyIdx + shift)
		)
		this.historyIdx = newIndex
		console.log({ oldIndex, newIndex })

		if (oldIndex == -1 && newIndex == -1) return
		else if (oldIndex == -1 && newIndex == 0) {
			this.stashedInput = elem.value
			elem.value = this.history[this.history.length - 1 - this.historyIdx]
		} else if (oldIndex == 0 && newIndex == -1) {
			elem.value = this.stashedInput ?? ''
			this.stashedInput == null
			return
		} else {
			elem.value = this.history[this.history.length - 1 - this.historyIdx]
		}

		this.refreshCompletions()
	}

	removeRange(start: number, end: number) {
		const elem = this.elem()
		if (!elem) return
		let val = elem.value
		elem.value = val.substring(0, start - 1) + val.substring(end)
	}

	pushChar(char: string) {
		const elem = this.elem()
		if (!elem) return
		if (this.historyIdx > -1) {
			this.stashedInput = null
			this.historyIdx = -1
		}
		console.log('start', elem.value)

		const selectStart = elem.selectionStart ?? elem.value.length - 1
		const newSelectStart = selectStart + char.length
		let start = elem.value.substring(0, selectStart)
		let end = elem.value.substring(selectStart)

		elem.value = start.concat(char, end)
		elem.selectionStart = newSelectStart
		elem.selectionEnd = newSelectStart
		console.log('end', elem.value)
	}

	/**
	 * Returns char index and word index of user selection position
	 * @returns [charIndex, wordIndex]
	 */
	getUserPosition(elem: HTMLInputElement): UserPositonState {
		const rawSplits = elem.value.split(' ')
		let offset = 0,
			wordIndex = 0,
			activeWord = ''
		let selectionStart = elem.selectionStart ?? elem.value.length - 1
		console.log('selectionStart', { selectionStart, rawSplits })

		for (const [i, part] of rawSplits.entries()) {
			console.log({ i, part })

			if (part.length > 0 || i == rawSplits.length - 1)
				if (selectionStart > offset && selectionStart <= offset + part.length) {
					wordIndex = i
					activeWord = part
					break
				}

			offset += part.length + 1
		}

		return {
			activeWord,
			wordIndex,
			wordStart: offset,
			wordEnd: offset + activeWord.length
		}
	}

	/**
	 * Given the current input value and user selection position,
	 * generate console prompt action and argument completions. Then, notify frontend
	 * of new completions using svelte stores.
	 */
	refreshCompletions(forceShow = false) {
		const elem = this.elem()
		if (!elem) return

		const completionInfo = wasm.completion(
			this.elem().value,
			this.elem().selectionStart ?? 0
		)
		if ('severity' in completionInfo) {
			handleClientError(completionInfo)
			return
		}

		const completionResult = completions.generateCompletions(completionInfo)
		console.log({ completionInfo, completionResult })

		this.wCompletions.set(completionResult)
	}

	submitConsoleComand(consoleCommand: string) {
		wConsole.echo(consoleCommand)
		this.pushToHistory(consoleCommand)
		this.historyIdx = -1
		this.clear()
		let result = wasm.action(consoleCommand)
		if ('message' in result) {
			handleClientError(result)
		} else {
			pushAction(result)
		}
	}

	pushToHistory(command: string) {
		const lastCommand = this.history.pop()
		if (!lastCommand || lastCommand == command) this.history.push(command)
		else this.history.push(lastCommand, command)
	}

	clear() {
		const elem = this.elem()
		if (!elem) return
		elem.value = ''
		this.wCompletions.set({
			completions: []
		})
	}
}

class InputController {
	private _map: { [key: string]: Input } = {}
	private activeInputKey: string | null = null

	private _wCompletions: Writable<Completions> = writable({
		completions: []
	})

	currentInput(): Input | null {
		return this.activeInputKey ? this._map[this.activeInputKey] : null
	}
	currentElem(): HTMLInputElement | null {
		return this.currentInput()?.elem() ?? null
	}

	register(inputRef: InputRef) {
		this._map[inputRef.key] = new Input(inputRef, this._wCompletions)
		console.log('registered ', inputRef.key)
	}

	deregister(key: string) {
		if (this.activeInputKey == key) this.deattach()
		delete this._map[key]
		console.log('deregistered', key)
	}

	attach(key: string) {
		this.deattach()
		const input = this._map[key]
		if (!input) {
			wConsole.debug('Cant attach, No input at key ' + key)
			return
		}
		input.attach()
		this.activeInputKey = key
	}

	deattach() {
		this.currentInput()?.deattach()
	}

	completions(): Readable<Completions> {
		return {
			subscribe: this._wCompletions.subscribe
		}
	}
}

export default new InputController()
