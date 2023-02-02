import { languages, Uri } from 'monaco-editor'
import type { LanguageServiceDefaults } from './index'
import * as service from './languageService'
import type WgslWorker from './wgslWorker'
import WorkerManager from './workerManager'


export function setupMode(
    defaults: LanguageServiceDefaults,
    modeId: string
): (...uris: Uri[]) => Promise<WgslWorker> {
    console.log('Setting up wgslMode', defaults, modeId)
    const client = new WorkerManager(modeId, defaults)
    const worker = (...uris: Uri[]): Promise<WgslWorker> => {
        return client.getLanguageServiceWorker(...uris)
    }

    languages.registerCompletionItemProvider(modeId, new service.SuggestAdapter(worker))
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
    // languages.registerReferenceProvider(
    //     modeId,
    //     new service.ReferenceAdapter(libFiles, worker)
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

    return worker
}