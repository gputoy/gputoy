import type * as monaco from 'monaco-editor'
import {
	COMPLETIONS_BUILTIN_FUNCTIONS,
	COMPLETIONS_BUILTIN_TYPES,
	COMPLETION_ATTRIBUTES,
	COMPLETION_KEYWORDS
} from './consts'
import type WgslWorker from './wgslWorker'

class ServiceBase {
	constructor(
		protected _worker: (...uris: monaco.Uri[]) => Promise<WgslWorker>
	) { }
}

export class SuggestAdapter
	extends ServiceBase
	implements monaco.languages.CompletionItemProvider {
	public get triggerCharacters(): string[] {
		return ['.', '@', ':']
	}
	async provideCompletionItems(
		model: monaco.editor.ITextModel,
		position: monaco.Position,
		context: monaco.languages.CompletionContext,
		token: monaco.CancellationToken
	): Promise<monaco.languages.CompletionList | undefined> {
		const wordInfo = model.getWordUntilPosition(position)

		const resource = model.uri
		const worker = await this._worker(resource)

		const prevPos = position.with(undefined, wordInfo.startColumn - 1)
		const prevWord = model.getWordUntilPosition(prevPos)
		const line = model.getLineContent(position.lineNumber)
		const [delim, isDelimBehindCursor] = this.getDelim(
			line,
			wordInfo.startColumn - 1
		)

		console.log({ curr: wordInfo.word, prev: prevWord.word, delim })

		if (model.isDisposed()) {
			return
		}

		switch (delim) {
			case '.': {
				// if the '.' is coming after a number
				if (!isNaN(prevWord.word as unknown as number)) {
					return {
						suggestions: [],
						incomplete: false
					}
				}
			}
			// return only attributes, all constant so we can skip wasm call
			case '@':
				return { suggestions: this.resetRanges(COMPLETION_ATTRIBUTES) }
			// return only types. TODO: add user defined types
			case ':': {
				let completions = COMPLETIONS_BUILTIN_TYPES
				// insert space before type
				if (isDelimBehindCursor)
					completions = completions.map((c) => ({
						...c,
						insertText: ' ' + c.insertText
					}))
				return {
					suggestions: this.resetRanges(completions)
				}
			}
		}
		// otherwise suggest keywords and functions
		const info = await worker.getCompletionsAtPosition(
			resource.path,
			prevWord.word,
			wordInfo.word,
			delim,
			position
		)
		const constSuggestions =
			COMPLETIONS_BUILTIN_FUNCTIONS.concat(COMPLETION_KEYWORDS)
		const ret = {
			suggestions: this.resetRanges(constSuggestions).concat(info ?? [])
		}
		return ret
	}
	async resolveCompletionItem?(
		item: monaco.languages.CompletionItem,
		token: monaco.CancellationToken
	): Promise<monaco.languages.CompletionItem> {
		return item
	}
	// Monaco actually writes to the constant completion item list
	// to assign an insert range. We just want the default every time.
	resetRanges(
		completions: monaco.languages.CompletionItem[]
	): monaco.languages.CompletionItem[] {
		for (const completion of completions) {
			/** @ts-ignore */
			delete completion['range']
		}
		return completions
	}
	getDelim(line: string, searchStart: number): [string, boolean] {
		let ptr = searchStart
		while (ptr--) {
			const char = line.charAt(ptr)
			console.log({ char, searchStart, ptr })
			if (char != ' ') {
				if (ptr == searchStart - 1) return [char, true]
				return [char, false]
			}
		}
		return ['', false]
	}
}

export class ReferenceAdapter
	extends ServiceBase
	implements monaco.languages.ReferenceProvider {
	async provideReferences(
		model: monaco.editor.ITextModel,
		position: monaco.Position,
		context: monaco.languages.ReferenceContext,
		token: monaco.CancellationToken
	): Promise<monaco.languages.Location[]> {
		const worker = await this._worker(model.uri)
		return []
		// return await worker.findReferences(model.uri.path, position)
	}
}
