import { pushAction } from '$core/actions'
import type { Completions } from '$core/completions'
import * as completions from '$core/completions'
import type { ConfigValueClass } from '$gen'
import { wConsole } from '$stores'
import * as wasm from '$wasm/common'
import { get, writable, type Readable, type Writable } from 'svelte/store'
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

class History {
    private history: string[] = []
    private historyIdx = -1
    private stashedInput: string | null = null

    constructor(
        private elem: HTMLInputElement
    ) {

    }

    shift(shift: number) {
        console.log('history', this.history)

        const elem = this.elem
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
    constructor(
        private ref: InputRef,
        private wCompletions: Writable<Completions>,
        private wCompletionsPosition: Writable<number>,
        private wInputBoundingBox: Writable<DOMRect | null>
    ) {
        this.history = new History(ref.elem)
    }

    elem(): HTMLInputElement {
        return this.ref.elem
    }

    attach() {
        const elem = this.elem()
        elem.onkeydown = this.onKeyDown.bind(this)
        elem.onkeypress = this.onKeyPress.bind(this)
        // elem.addEventListener('keypress', this.onKeyPress.bind(this))
    }

    deattach() {
        const elem = this.elem()
        elem.onkeydown = null
        elem.onkeypress = null
        // elem.removeEventListener('keydown', this.onKeyDown)
        // elem.removeEventListener('keypress', this.onKeyDown)
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
                this.history.shift(1)
                this.refreshCompletions()
                event.preventDefault()
                break
            case 'ArrowDown':
                this.history.shift(-1)
                this.refreshCompletions()
                event.preventDefault()
                break
            case 'ArrowLeft':
            case 'ArrowRight':
                this.refreshCompletions()
                break
            case 'Enter':
                console.log(this.ref.class)

                if (this.ref.class.ty == 'StrClass') {
                    this.ref.onChange(this.ref.elem.value)
                } else if (this.ref.class.ty == 'CmdClass') {
                    this.submitConsoleComand(this.ref.elem.value)
                }
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

    removeRange(start: number, end: number) {
        const elem = this.elem()
        if (!elem) return
        let val = elem.value
        elem.value = val.substring(0, start - 1) + val.substring(end)
    }

    moveCompletionIndex(shift: number, userPosition: UserPositonState) {
        const elem = this.elem()
        const completions = get(this.wCompletions)
        const len = completions.completions.length
        this.wCompletionsPosition.update(index => {
            let newIndex = index + shift
            if (newIndex < 0) newIndex = Math.max(-1, newIndex + len)
            return newIndex % len
        })
        const completion = completions.completions[get(this.wCompletionsPosition)]
        if (completion) {
            let insertText = completion.insertText
            let start = elem.value.substring(0, userPosition.wordStart)
            let end = elem.value.substring(userPosition.wordEnd)
            elem.value = start.concat(insertText, end)
        }
    }




    pushChar(char: string) {
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
    }

    /**
     * Returns char index and word index of user selection position
     * @returns [charIndex, wordIndex]
     */
    getUserPosition(elem: HTMLInputElement): UserPositonState {
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
        const elem = this.elem()
        if (!elem) return
        const value = elem.value.trim()
        if (value.length == 0 && !forceShow) {
            this.wCompletions.set({
                completions: []
            })
            return
        }
        this.wInputBoundingBox.set(elem.getBoundingClientRect())

        const completionInfo = wasm.completion(
            value,
            elem.selectionStart ?? 0
        )
        if ('severity' in completionInfo) {
            handleClientError(completionInfo)
            return
        }

        const completionResult = completions.generateCompletions(completionInfo)
        // update the completions position so it resets if the list is empty
        this.wCompletionsPosition.update(pos => Math.min(completionResult.completions.length - 1, pos))
        this.wCompletions.set(completionResult)
    }

    submitConsoleComand(consoleCommand: string) {
        wConsole.echo(consoleCommand)
        this.history.push(consoleCommand)
        this.history.reset()
        this.clear()
        let result = wasm.action(consoleCommand)
        if ('severity' in result) {
            handleClientError(result)
        } else {
            pushAction(result)
        }
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
    private _wCompletionsIndex: Writable<number> = writable(-1)
    private _wCompletionsLocation: Writable<DOMRect | null> = writable(null)

    currentInput(): Input | null {
        return this.activeInputKey ? this._map[this.activeInputKey] : null
    }
    currentElem(): HTMLInputElement | null {
        return this.currentInput()?.elem() ?? null
    }

    register(inputRef: InputRef) {
        this._map[inputRef.key] = new Input(inputRef, this._wCompletions, this._wCompletionsIndex, this._wCompletionsLocation)
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
    }

    deattach() {
        this.currentInput()?.deattach()
        this.activeInputKey = null
    }

    completions(): Readable<Completions> {
        return {
            subscribe: this._wCompletions.subscribe
        }
    }
    completionsIndex(): Readable<number> {
        return {
            subscribe: this._wCompletionsIndex.subscribe
        }
    }
    completionsLocation(): Readable<DOMRect | null> {
        return {
            subscribe: this._wCompletionsLocation.subscribe
        }
    }
}

export default new InputController()
