import * as completions from '$core/completions'
import type { ArgDescriptor } from '$gen'
import { wConsole } from '$stores'

type UserPositonState = {
	activeWord: string
	wordIndex: number
	wordStart: number
	wordEnd: number
}

type InputRef = {
	key: string
	elem: HTMLInputElement
	descriptor?: ArgDescriptor
	onSubmit: (val: any) => void
	onChange?: (val: any) => void
}

class History {
	private history: string[] = []
	private historyIdx = -1
	private stashedInput: string | null = null

	constructor(private elem: HTMLInputElement) {}

	shift(shift: number) {
		const elem = this.elem
		if (!elem) return
		let oldIndex = this.historyIdx
		let newIndex = Math.min(
			this.history.length - 1,
			Math.max(-1, this.historyIdx + shift)
		)
		this.historyIdx = newIndex
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
	}

	push(command: string) {
		const lastCommand = this.history.pop()
		if (!lastCommand || lastCommand == command) this.history.push(command)
		else this.history.push(lastCommand, command)
	}

	reset() {
		this.historyIdx = -1
		this.stashedInput = null
	}
}

class Input {
	private history: History
	constructor(private ref: InputRef) {
		this.history = new History(ref.elem)
	}

	elem(): HTMLInputElement {
		return this.ref.elem
	}

	attach() {
		const elem = this.elem()
		elem.onkeydown = this.onKeyDown.bind(this)
		elem.onkeypress = this.onKeyPress.bind(this)
	}

	deattach() {
		const elem = this.elem()
		elem.onkeydown = null
		elem.onkeypress = null
	}

	private onKeyDown(event: KeyboardEvent) {
		const elem = this.elem()
		if (!elem) return
		switch (event.key) {
			case 'ArrowLeft':
			case 'ArrowRight':
				this.refreshCompletions()
				break
			case 'Escape':
				completions.reset()
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
				this.history.shift(1)
				this.refreshCompletions()
				event.preventDefault()
				break
			case 'ArrowDown':
				this.history.shift(-1)
				this.refreshCompletions()
				event.preventDefault()
				break
			case 'Enter':
				this.history.push(this.ref.elem.value)
				this.history.reset()
				this.ref.onSubmit(this.ref.elem.value)
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
			// event.stopImmediatePropagation()
		}
	}

	private onChange(value: any) {
		if (this.ref.onChange) this.ref.onChange(value)
	}

	private removeRange(start: number, end: number) {
		const elem = this.elem()
		if (!elem) return
		let val = elem.value
		let newValue = val.substring(0, start - 1) + val.substring(end)
		if (newValue == elem.value) return
		elem.value = newValue
		this.onChange(elem.value)
	}

	private moveCompletionIndex(shift: number, userPosition: UserPositonState) {
		const elem = this.elem()
		const match = completions.shiftCompletionIndex(shift)
		if (match) {
			let insertText = match.insertText
			let start = elem.value.substring(0, userPosition.wordStart)
			let end = elem.value.substring(userPosition.wordEnd)
			elem.value = start.concat(insertText, end)
			this.onChange(elem.value)
		}
	}

	private pushChar(char: string) {
		const elem = this.elem()
		if (!elem) return
		this.history.reset()
		const selectStart = elem.selectionStart ?? elem.value.length - 1
		const newSelectStart = selectStart + char.length
		let start = elem.value.substring(0, selectStart)
		let end = elem.value.substring(selectStart)

		elem.value = start.concat(char, end)
		elem.selectionStart = newSelectStart
		elem.selectionEnd = newSelectStart
		this.onChange(elem.value)
	}

	/**
	 * Returns char index and word index of user selection position
	 * @returns [charIndex, wordIndex]
	 */
	private getUserPosition(elem: HTMLInputElement): UserPositonState {
		const rawSplits = elem.value.split(' ')
		let offset = 0,
			wordIndex = 0,
			activeWord = '',
			selectionStart = elem.selectionStart ?? elem.value.length - 1

		for (const [i, part] of rawSplits.entries()) {
			if (part.length > 0 || i == rawSplits.length - 1) {
				if (selectionStart > offset && selectionStart <= offset + part.length) {
					wordIndex = i
					activeWord = part
					break
				}
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
		setTimeout(() => this._refreshCompletions(forceShow), 10)
	}

	private _refreshCompletions(forceShow: boolean) {
		const elem = this.elem()
		if (!elem) return
		if (elem.value.length == 0 && !forceShow) {
			completions.reset()
			return
		}
		completions.updateLocationFromDomRect(elem.getBoundingClientRect())
		const { value, selectionStart } = elem
		if (this.ref.descriptor) {
			completions.regenerateCompletions(
				value,
				selectionStart ?? 0,
				this.ref.descriptor
			)
		} else {
			completions.regenerateCompletions(value, selectionStart ?? 0)
		}
	}

	clear() {
		const elem = this.elem()
		if (!elem) return
		elem.value = ''
		completions.reset()
	}
}

class InputController {
	private _map: { [key: string]: Input } = {}
	private activeInputKey: string | null = null

	get(key: string): Input | null {
		return this._map[key] ?? null
	}

	currentInput(): Input | null {
		return this.activeInputKey ? this._map[this.activeInputKey] : null
	}
	currentElem(): HTMLInputElement | null {
		return this.currentInput()?.elem() ?? null
	}

	register(inputRef: InputRef) {
		this._map[inputRef.key] = new Input(inputRef)
	}

	deregister(key: string) {
		if (this.activeInputKey == key) this.deattach()
		delete this._map[key]
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
		completions.updateLocationFromDomRect(input.elem().getBoundingClientRect())
		input.refreshCompletions()
	}

	deattach() {
		this.currentInput()?.deattach()
		this.activeInputKey = null
		completions.reset()
	}
}

export default new InputController()
