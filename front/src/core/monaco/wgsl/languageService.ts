import * as monaco from 'monaco-editor'
import type WgslWorker from './wgslWorker'

export class SuggestAdapter implements monaco.languages.CompletionItemProvider {
    constructor(protected _worker: (...uris: monaco.Uri[]) => Promise<WgslWorker>) { }
    public get triggerCharacters(): string[] {
        return ['.', '@', '<', '(']
    }
    async provideCompletionItems(
        model: monaco.editor.ITextModel,
        position: monaco.Position,
        context: monaco.languages.CompletionContext,
        token: monaco.CancellationToken
    ): Promise<monaco.languages.CompletionList | undefined> {
        const wordInfo = model.getWordUntilPosition(position)
        console.log({ model, position, context, token, wordInfo })
        const wordRange = new monaco.Range(
            position.lineNumber,
            wordInfo.startColumn,
            position.lineNumber,
            wordInfo.endColumn
        )
        const resource = model.uri
        const offset = model.getOffsetAt(position)

        const worker = await this._worker(resource)

        if (model.isDisposed()) {
            return
        }

        // const info = await worker.getCompletionsAtPosition(resource.toString(), offset)

        // if (!info || model.isDisposed()) {
        //     return
        // }

        // const suggestions: monaco.languages.CompltionItem[] = info.entries.map((entry) => {
        //     let range = wordRange
        //     if (entry.replacementSpan) {
        //         const p1 = model.getPositionAt(entry.replacementSpan.start)
        //         const p2 = model.getPositionAt(entry.replacementSpan.start + entry.replacementSpan.length)
        //         range = new Range(p1.lineNumber, p1.column, p2.lineNumber, p2.column)
        //     }

        //     const tags: languages.CompletionItemTag[] = []
        //     if (entry.kindModifiers?.indexOf('deprecated') !== -1) {
        //         tags.push(languages.CompletionItemTag.Deprecated)
        //     }

        //     return {
        //         uri: resource,
        //         position: position,
        //         offset: offset,
        //         range: range,
        //         label: entry.name,
        //         insertText: entry.name,
        //         sortText: entry.sortText,
        //         kind: SuggestAdapter.convertKind(entry.kind),
        //         tags
        //     }
        // })

        // return {
        //     suggestions
        // }
        return {
            suggestions: allCompletions,
        }
    }
    async resolveCompletionItem?(
        item: monaco.languages.CompletionItem,
        token: monaco.CancellationToken
    ): Promise<monaco.languages.CompletionItem> {
        return item
    }

}

const attributeCompletions: monaco.languages.CompletionItem[] = [
    {
        insertText: 'export',
        label: '@export',
        kind: 17,
        documentation: 'Export this type to global namespace.'
    }
]

const keywordSuggestions: monaco.languages.CompletionItem[] = [
    "bitcast",
    "break",
    "case",
    "continue",
    "continuing",
    "default",
    "discard",
    "else",
    "enable",
    "fallthrough",
    "fn",
    "for",
    "if",
    "loop",
    "private",
    "read",
    "read_write",
    "return",
    "storage",
    "struct",
    "switch",
    "type",
    "uniform",
    "workgroup",
    "write",
    "stage",
    "workgroup_size",
    "group",
    "binding",
    "block",
]
    .map((w) => {
        return { label: w, insertText: w, kind: 17, documentation: "keyword" }
    })
    .concat([
        {
            insertText: "let",
            label: "let",
            kind: 17,
            documentation:
                "A let declaration specifies a name for a value. Once the value for a let-declaration is computed, it is immutable. When an identifier use resolves to a let-declaration, the identifier denotes that value.",
        },
        {
            insertText: "var",
            label: "var",
            kind: 17,
            documentation:
                "A variable is a named reference to memory that can contain a value of a particular storable type.",
        },
    ])

const allCompletions = keywordSuggestions.concat(attributeCompletions)