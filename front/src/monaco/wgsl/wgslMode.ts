import { wConfig, wModelDirty } from '$stores'
import * as monaco from 'monaco-editor'
import {
	get,
	writable,
	type Readable,
	type Subscriber,
	type Unsubscriber,
	type Writable
} from 'svelte/store'
import type { LanguageServiceDefaults } from './index'
import * as service from './languageService'
import type WgslWorker from './wgslWorker'
import WorkerManager from './workerManager'

let _client: WorkerManager
export var wWorkerInternal: WorkerInternalReadable

type WorkerAccessor = (...uris: monaco.Uri[]) => Promise<WgslWorker>

type Label = {
	/// The style of the label.
	style: 'Primary' | 'Secondary'
	/// The file that we are labelling.
	file_id: string
	/// The range in bytes we are going to include in the final snippet.
	range: {
		start: number
		end: number
	}
	/// An optional message to provide some additional information for the
	/// underlined code. These should not include line breaks.
	message: string
}

class WorkerInternalReadable implements Readable<any> {
	private _store: Writable<any>
	constructor(private _worker: WorkerAccessor) {
		this._store = writable<any>({})
	}

	subscribe(run: Subscriber<any>, invalidate?: any): Unsubscriber {
		return this._store.subscribe(run, invalidate)
	}

	async poll(): Promise<any> {
		const service = await this._worker()
		const newData = Object.fromEntries(await service.expose())
		this._store.set(newData)
		return newData
	}

	async applyUpdateToFile(uri?: string): Promise<void> {
		const service = await this._worker()
		service.applyChanges(uri)
	}

	async tryBuild(): Promise<void> {
		const service = await this._worker()
		const runnerUri = get(wConfig).runner ?? ''
		const result = await service.tryBuild(runnerUri)

		if ('diagnostics' in result) {
			const diagnostics = result.diagnostics

			// fileid (string) => monaco marker array
			const monacoDiagnostics: { [key: string]: monaco.editor.IMarkerData[] } =
				{}

			// fileid (string) => monaco model
			const models = new Map(
				monaco.editor.getModels().map((model) => [model.uri.path, model])
			)
			for (const diagnostic of diagnostics) {
				diagnostic.labels.filterMap((label: Label) => {
					const model = models.get(label.file_id)
					if (!model) return
					const start = model.getPositionAt(label.range.start)
					const end = model.getPositionAt(label.range.end)

					const marker: monaco.editor.IMarkerData = {
						startColumn: start.column,
						endColumn: end.column,
						startLineNumber: start.lineNumber,
						endLineNumber: end.lineNumber,
						message: label.message,
						severity: diagnostic.severity
					}
					return marker
				})
			}
			for (const [fileid, diagnostics] of Object.entries(monacoDiagnostics)) {
				const model = models.get(fileid)
				if (!model) return
				monaco.editor.setModelMarkers(model, 'test-owner', diagnostics)
			}
			console.info('tryBuild result: ', result)
		} else {
			console.error('tryBuild error: ', result)
		}
	}
}

export function setupMode(
	defaults: LanguageServiceDefaults,
	modeId: string
): WorkerAccessor {
	_client = new WorkerManager(modeId, defaults)
	const worker = (...uris: monaco.Uri[]): Promise<WgslWorker> => {
		return _client.getLanguageServiceWorker(...uris)
	}

	wWorkerInternal = new WorkerInternalReadable(worker)

	setupModelHandlers(worker)
	miscServiceHandlers(worker)

	monaco.languages.registerCompletionItemProvider(
		modeId,
		new service.SuggestAdapter(worker)
	)
	monaco.languages.registerReferenceProvider(
		modeId,
		new service.ReferenceAdapter(worker)
	)

	return worker
}

function setupModelHandlers(worker: WorkerAccessor) {
	const createOnDidChangeContentHandler = (
		model: monaco.editor.ITextModel
	): ((ev: monaco.editor.IModelContentChangedEvent) => Promise<void>) => {
		return async (ev: monaco.editor.IModelContentChangedEvent) => {
			const wasmWorker = await worker()
			// TODO: do something on error instead of failing silently
			const path = model.uri.path
			const [versionCurrent, versionPatch] = (await wasmWorker.handleFileDelta(
				path,
				ev
			)) ?? [0, 0]
			if (versionCurrent != versionPatch) {
				wModelDirty.add(path)
			}
		}
	}

	monaco.editor.onDidCreateModel((model) =>
		model.onDidChangeContent(createOnDidChangeContentHandler(model))
	)
	monaco.editor
		.getModels()
		.forEach((model) =>
			model.onDidChangeContent(createOnDidChangeContentHandler(model))
		)
	monaco.editor.onWillDisposeModel((model) => console.log('disposing: ', model))
}

export function dispose() {
	_client?.dispose()
}

// TODO
function miscServiceHandlers(_worker: WorkerAccessor) {
	// languages.registerSignatureHelpProvider(
	//     modeId,
	//     new service.SignatureHelpAdapter(worker)
	// )
	// languages.registerHoverProvider(modeId, new service.QuickInfoAdapter(worker))
	// languages.registerDocumentHighlightProvider(
	//     modeId,
	//     new service.OccurrencesAdapter(worker)
	// )
	// languages.registerDefinitionProvider(
	//     modeId,
	//     new service.DefinitionAdapter(libFiles, worker)
	// )
	// languages.registerDocumentSymbolProvider(modeId, new service.OutlineAdapter(worker))
	// languages.registerDocumentRangeFormattingEditProvider(
	//     modeId,
	//     new service.FormatAdapter(worker)
	// )
	// languages.registerOnTypeFormattingEditProvider(
	//     modeId,
	//     new service.FormatOnTypeAdapter(worker)
	// )
	// languages.registerCodeActionProvider(modeId, new service.CodeActionAdaptor(worker))
	// languages.registerRenameProvider(modeId, new service.RenameAdapter(libFiles, worker))
	// languages.registerInlayHintsProvider(modeId, new service.InlayHintsAdapter(worker))
	// new service.DiagnosticsAdapter(libFiles, defaults, modeId, worker)
}
