import { Compiler as WasmCompiler, default as init_compiler } from '$wasm/compiler/gpu_wasm_compiler'
import type * as monaco from 'monaco-editor'

export interface ICreateData {
}

export type Diagnostic = {}
export type CompletionList = {}
export type Hover = {}
export type Location = {}
export type DocumentHighlight = {}
export type SymbolInformation = {}
export type CompletionInfo = {}

export default class WgslWorker {

    private ctx: monaco.worker.IWorkerContext
    private wasm_service!: WasmCompiler
    private language_id: string

    constructor(ctx: monaco.worker.IWorkerContext, createData: ICreateData) {

        console.log('Constructing gputoy worker', ctx, createData)
        this.ctx = ctx

        init_compiler().then(() =>
            this.wasm_service = new WasmCompiler()
        )

        this.language_id = ""
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
    ): Promise<Location | null> {
        console.log('findDefinition', uri, position)
        return Promise.resolve(null)
    }
    async findReferences(uri: string, position: monaco.Position): Promise<Location[]> {
        console.log('findReferences', uri, position)
        return Promise.resolve([])
    }
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
        context: CodeActionContext
    ): Promise<Command[]> {
        console.log('doCodeACtions', uri, range, context)
        return Promise.resolve([])
    }
    async getFoldingRanges(
        uri: string,
        context?: { rangeLimit?: number }
    ): Promise<FoldingRange[]> {
        console.log('getFoldingRanges', uri, context)
        return Promise.resolve([])
    }
    async getCompletionsAtPosition(
        uri: string,
        position: number
    ): Promise<CompletionInfo | undefined> {
        console.log('getCompletionsAtPosition', uri, position)
        return undefined
    }
    async getSelectionRanges(
        uri: string,
        positions: monaco.Position[]
    ): Promise<SelectionRange[]> {
        console.log('getSelectionRanges', uri, positions)
        return Promise.resolve([])
    }
    async doRename(
        uri: string,
        position: monaco.Position,
        newName: string
    ): Promise<WorkspaceEdit | null> {
        console.log('doRename', uri, position, newName)
        return Promise.resolve(null)
    }
    async format(
        uri: string,
        range: monaco.Range | null,
        options: FormatConfig
    ): Promise<TextEdit[]> {
        console.log('format', uri, range, options)
        return Promise.resolve([])
    }
    private _getTextDocument(uri: string): string | null {
        const models = this.ctx.getMirrorModels()
        for (const model of models) {
            if (model.uri.toString() === uri) {
                return model.getValue()
            }
        }
        return null
    }
}

// URI path utilities, will (hopefully) move to vscode-uri

const Slash = '/'.charCodeAt(0)
const Dot = '.'.charCodeAt(0)

function isAbsolutePath(path: string) {
    return path.charCodeAt(0) === Slash
}

function resolvePath(uriString: string, path: string): string {
    if (isAbsolutePath(path)) {
        const uri = URI.parse(uriString)
        const parts = path.split('/')
        return uri.with({ path: normalizePath(parts) }).toString()
    }
    return joinPath(uriString, path)
}

function normalizePath(parts: string[]): string {
    const newParts: string[] = []
    for (const part of parts) {
        if (part.length === 0 || (part.length === 1 && part.charCodeAt(0) === Dot)) {
            // ignore
        } else if (part.length === 2 && part.charCodeAt(0) === Dot && part.charCodeAt(1) === Dot) {
            newParts.pop()
        } else {
            newParts.push(part)
        }
    }
    if (parts.length > 1 && parts[parts.length - 1].length === 0) {
        newParts.push('')
    }
    let res = newParts.join('/')
    if (parts[0].length === 0) {
        res = '/' + res
    }
    return res
}

function joinPath(uriString: string, ...paths: string[]): string {
    const uri = Uri.parse(uriString)
    const parts = uri.path.split('/')
    for (let path of paths) {
        parts.push(...path.split('/'))
    }
    return uri.with({ path: normalizePath(parts) }).toString()
}

export interface ICreateData {
    languageId: string
    languageSettings: DiagnosticsOptions
    enableSchemaRequest: boolean
}

export function create(ctx: worker.IWorkerContext, createData: ICreateData): WgslWorker {
    return new WgslWorker(ctx, createData)
}