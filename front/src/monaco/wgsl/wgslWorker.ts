/**
 * Its very important that monaco is NOT imported like this:
 *
 * `import * as monaco from 'monaco-editor'`
 *
 * Doing so will pollute the worker with tons of css that vite will handily
 * convert to hot-reload js modules that try to access the DOM.
 *
 * Feb 5th, 2023
 */

import type { Files } from '$core/common'
import {
	Analyzer as WasmAnalyzer,
	default as init_analyzer,
	Position
} from '$wasm/analyzer/gpu_wasm_analyzer'
import type * as monaco from 'monaco-editor'

export type Diagnostic = {}
export type CompletionList = {}
export type Hover = {}
export type DocumentHighlight = {}
export type SymbolInformation = {}
export type CompletionInfo = {}

export default class WgslWorker {
	private ctx: monaco.worker.IWorkerContext
	private wasm_service!: monaco.Thenable<WasmAnalyzer>
	private language_id: string

	constructor(ctx: monaco.worker.IWorkerContext, createData: CreateData) {
		this.ctx = ctx

		this.wasm_service = init_analyzer(createData.wasmModuleUrl).then(() => {
			return WasmAnalyzer.new(createData.initFiles)
		})

		this.language_id = createData.languageId
	}
	handleWasmError(e: any) {
		console.warn('Encountered wasm error: ', e)
	}
	async expose(): Promise<any> {
		const service = await this.wasm_service
		return service.expose()
	}
	async tryBuild(runnerUri: string): Promise<any> {
		const service = await this.wasm_service
		try {
			return service.try_build(runnerUri)
		} catch (e) {
			this.handleWasmError(e)
		}
	}
	async handleFileDelta(
		uri: string,
		event: monaco.editor.IModelContentChangedEvent
	): Promise<[number, number] | undefined> {
		const service = await this.wasm_service
		let version
		try {
			if (event.changes.length == 1 && event.changes[0].text.length == 1) {
				const change = event.changes[0]
				version = service.handleFileDeltaChar(
					uri,
					change.text,
					change.rangeOffset,
					event.versionId
				)
			} else {
				version = service.handleFileDelta(
					uri,
					event.changes,
					event.versionId,
					event.isFlush
				)
			}
			return [version.current, version.patched]
		} catch (e) {
			this.handleWasmError(e)
		} finally {
			// version?.free()
		}
	}
	async applyChanges(uri?: string) {
		const service = await this.wasm_service
		try {
			service.applyFileChanges(uri)
		} catch (e) {
			this.handleWasmError(e)
		}
	}
	async doValidation(uri: string): Promise<Diagnostic[]> {
		console.log('doValidation: ', uri)
		return Promise.resolve([])
	}
	async doComplete(
		uri: string,
		position: monaco.Position
	): Promise<CompletionList | null> {
		console.log('doComplete: ', uri)
		return Promise.resolve(null)
	}
	async doHover(uri: string, position: monaco.Position): Promise<Hover | null> {
		console.log('doHover: ', uri, position)
		return Promise.resolve(null)
	}
	async findDefinition(
		uri: string,
		position: monaco.Position
	): Promise<monaco.languages.Location | null> {
		console.log('findDefinition', uri, position)
		return Promise.resolve(null)
	}
	// async findReferences(
	// 	uri: string,
	// 	position: monaco.Position
	// ): Promise<monaco.languages.Location[]> {
	// 	console.log('findReferences', uri, position)
	// 	return Promise.resolve([{}])
	// }
	async findDocumentHighlights(
		uri: string,
		position: monaco.Position
	): Promise<DocumentHighlight[]> {
		console.log('findDocumentHighlights', uri, position)
		return Promise.resolve([])
	}
	async findDocumentSymbols(uri: string): Promise<SymbolInformation[]> {
		console.log('findDocumentSymbol', uri)
		return Promise.resolve([])
	}
	async doCodeActions(
		uri: string,
		range: monaco.Range,
		/** @ts-ignore */
		context: CodeActionContext
		/** @ts-ignore */
	): Promise<Command[]> {
		console.log('doCodeACtions', uri, range, context)
		return Promise.resolve([])
	}
	async getCompletionsAtPosition(
		uri: string,
		prevWord: string,
		currWord: string,
		delim: string,
		position: monaco.Position
	): Promise<monaco.languages.CompletionItem[] | undefined> {
		const service = await this.wasm_service
		const wasmPosition = new Position(position.column, position.lineNumber)
		try {
			const result = service.getCompletionsAtPosition(
				uri,
				prevWord,
				currWord,
				delim,
				wasmPosition
			)
			return result
		} catch (e) {
			this.handleWasmError(e)
		}
	}
	async getSelectionRanges(
		uri: string,
		positions: monaco.Position[]
		/** @ts-ignore */
	): Promise<SelectionRange[]> {
		console.log('getSelectionRanges', uri, positions)
		return Promise.resolve([])
	}
	async doRename(
		uri: string,
		position: monaco.Position,
		newName: string
		/** @ts-ignore */
	): Promise<WorkspaceEdit | null> {
		console.log('doRename', uri, position, newName)
		return Promise.resolve(null)
	}
	async format(
		uri: string,
		range: monaco.Range | null,
		/** @ts-ignore */
		options: FormatConfig
		/** @ts-ignore */
	): Promise<TextEdit[]> {
		console.log('format', uri, range, options)
		return Promise.resolve([])
	}
	private _getModel(uri: string): monaco.worker.IMirrorModel | null {
		const models = this.ctx.getMirrorModels()
		for (const model of models) {
			if (model.uri.toString() === uri) {
				return model
			}
		}
		return null
	}
}

export interface CreateData {
	languageId: string
	/** @ts-ignore */
	languageSettings: DiagnosticsOptions
	enableSchemaRequest: boolean
	wasmModuleUrl: string,
	initFiles: Files
}

export function create(
	ctx: monaco.worker.IWorkerContext,
	createData: CreateData
): WgslWorker {
	console.log('create called')
	return new WgslWorker(ctx, createData)
}
