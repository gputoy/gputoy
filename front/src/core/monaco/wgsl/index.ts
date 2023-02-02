import { Emitter, languages, type IEvent } from "monaco-editor"

export const id = "wgsl"
export const extensionPoint: languages.ILanguageExtensionPoint = {
    id,
}

export const config: languages.LanguageConfiguration = {
    comments: {
        lineComment: "//",
        blockComment: ["/*", "*/"],
    },
    brackets: [
        ["{", "}"],
        ["[", "]"],
        ["(", ")"],
        ["<", ">"],
    ],
    autoClosingPairs: [
        { open: "[", close: "]" },
        { open: "{", close: "}" },
        { open: "(", close: ")" },
        { open: "<", close: ">" },
    ],
    surroundingPairs: [
        { open: "{", close: "}" },
        { open: "[", close: "]" },
        { open: "(", close: ")" },
        { open: "<", close: ">" },
    ],
}
export const monarch: languages.IMonarchLanguage = {
    defaultToken: "invalid",

    keywords: [
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
        "let",
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
        "var",
        "workgroup",
        "write",
        "stage",
        "workgroup_size",
        "group",
        "binding",
        "block",
    ],

    typeKeywords: [
        "array",
        "atomic",
        "ptr",
        "sampler",
        "sampler_comparison",
        "texture_1d",
        "texture_2d",
        "texture_3d",
        "texture_2d_array",
        "texture_cube",
        "texture_cube_array",
        "texture_multisampled_2d",
        "texture_storage_1d",
        "texture_storage_2d",
        "texture_storage_2d_array",
        "texture_storage_3d",
        "texture_depth_2d",
        "texture_depth_2d_array",
        "texture_depth_cube",
        "texture_depth_cube_array",
        "texture_depth_multisampled_2d",
        "f32",
        "i32",
        "bool",
        "u32",
        "vec2",
        "vec3",
        "vec4",
        "mat2x2",
        "mat2x3",
        "mat2x4",
        "mat3x2",
        "mat3x3",
        "mat3x4",
        "mat4x2",
        "mat4x3",
        "mat4x4",
    ],

    constants: ["true", "false", "compute", "fragment", "vertex"],

    operators: [
        "=",
        ">",
        "<",
        "!",
        "~",
        "?",
        ":",
        "==",
        "<=",
        ">=",
        "!=",
        "&&",
        "||",
        "++",
        "--",
        "+",
        "-",
        "*",
        "/",
        "&",
        "|",
        "^",
        "%",
        "<<",
        ">>",
        ">>>",
    ],

    // brackets: [['{', '}', 'delimiter.curly'],
    // ['[', ']', 'delimiter.square'],
    // ['(', ')', 'delimiter.parenthesis'],
    // ['<', '>', 'delimiter.angle']],

    // we include these common regular expressions
    symbols: /[\#\!\%\&\*\+\-\.\/\:\;\<\=\>\@\^\|_\?]+/,

    escapes: /\\([nrt0\"''\\]|x\h{2}|u\{\h{1,6}\})/,

    // The main tokenizer for our languages
    tokenizer: {
        root: [
            [/@[a-z]+/g, "attributes"],
            [/[A-Z][\w\$]*/g, "struct"],
            [/\b(([a-z|_][\w]+)|([a-z][\w]*))(?=\()/, "function"],
            [/#[\w]+\b/, "params"],
            // identifiers and keywords
            [
                /([a-zA-Z_][0-9a-zA-Z][0-9a-zA-Z_]*)|([a-zA-Z][0-9a-zA-Z_]*)/,
                {
                    cases: {
                        "@typeKeywords": "typeKeyword",
                        "@keywords": "keyword",
                        "@constants": "constants",
                        "@default": "identifier",
                    },
                },
            ],

            // whitespace
            { include: "@whitespace" },

            // delimiters and operators
            [/[{}()\[\]]/, "@brackets"],
            [/[<>](?!@symbols)/, "@brackets"],
            [
                /@symbols/,
                {
                    cases: {
                        "@operators": "operator",
                        "@default": "",
                    },
                },
            ],

            // @ annotations.
            // As an example, we emit a debugging log message on these tokens.
            // Note: message are supressed during the first load -- change some lines to see them.
            [/@\s*[a-zA-Z_\$][\w\$]*/, { token: "annotation" }],

            // numbers
            [/\d*\.\d+([eE][\-+]?\d+)?/, "number.float"],
            [/0[xX][0-9a-fA-F]+/, "number.hex"],
            [/\d+/, "number"],

            // delimiter: after number because of .\d floats
            [/[;,.]/, "delimiter"],

            // strings
            [/"([^"\\]|\\.)*$/, "string.invalid"], // non-teminated string
            [/"/, { token: "string.quote", bracket: "@open", next: "@string" }],

            // characters
            [/'[^\\']'/, "string"],
            [/(')(@escapes)(')/, ["string", "string.escape", "string"]],
            [/'/, "string.invalid"],
        ],

        comment: [
            [/[^\/*]+/, "comment"],
            [/\/\*/, "comment", "@push"], // nested comment
            ["\\*/", "comment", "@pop"],
            [/[\/*]/, "comment"],
        ],

        string: [
            [/[^\\"]+/, "string"],
            [/@escapes/, "string.escape"],
            [/\\./, "string.escape.invalid"],
            [/"/, { token: "string.quote", bracket: "@close", next: "@pop" }],
        ],
        whitespace: [
            [/[ \t\r\n]+/, "white"],
            [/\/\*/, "comment", "@comment"],
            [/\/\/.*$/, "comment"],
        ],
    },
}

export type SeverityLevel = 'ignore' | 'warn' | 'error'

export interface DiagnosticsOptions {
}

export interface LanguageServiceDefaults {
    readonly languageId: string
    readonly onDidChange: IEvent<LanguageServiceDefaults>
    readonly diagnosticsOptions: DiagnosticsOptions
    readonly modeConfiguration: ModeConfiguration
    setDiagnosticsOptions(options: DiagnosticsOptions): void
    setModeConfiguration(modeConfiguration: ModeConfiguration): void
}

export interface ModeConfiguration {
    /**
     * Defines whether the built-in completionItemProvider is enabled.
     */
    readonly completionItems?: boolean

    /**
     * Defines whether the built-in hoverProvider is enabled.
     */
    readonly hovers?: boolean

    /**
     * Defines whether the built-in documentSymbolProvider is enabled.
     */
    readonly documentSymbols?: boolean

    /**
     * Defines whether the built-in definitions provider is enabled.
     */
    readonly links?: boolean

    /**
     * Defines whether the built-in references provider is enabled.
     */
    readonly documentHighlights?: boolean

    /**
     * Defines whether the built-in rename provider is enabled.
     */
    readonly rename?: boolean

    /**
     * Defines whether the built-in color provider is enabled.
     */
    readonly colors?: boolean

    /**
     * Defines whether the built-in foldingRange provider is enabled.
     */
    readonly foldingRanges?: boolean

    /**
     * Defines whether the built-in diagnostic provider is enabled.
     */
    readonly diagnostics?: boolean

    /**
     * Defines whether the built-in selection range provider is enabled.
     */
    readonly selectionRanges?: boolean

    /**
     * Defines whether the built-in documentFormattingEdit provider is enabled.
     */
    readonly documentFormattingEdits?: boolean

    /**
     * Defines whether the built-in documentRangeFormattingEdit provider is enabled.
     */
    readonly documentRangeFormattingEdits?: boolean
}

export interface LanguageServiceDefaults {
    readonly languageId: string
    readonly onDidChange: IEvent<LanguageServiceDefaults>
    readonly diagnosticsOptions: DiagnosticsOptions
    readonly modeConfiguration: ModeConfiguration
    setDiagnosticsOptions(options: DiagnosticsOptions): void
    setModeConfiguration(modeConfiguration: ModeConfiguration): void
}

class LanguageServiceDefaultsImpl implements LanguageServiceDefaults {
    private _onDidChange = new Emitter<LanguageServiceDefaults>();
    private _diagnosticsOptions!: DiagnosticsOptions
    private _modeConfiguration!: ModeConfiguration
    private _languageId: string

    constructor(
        languageId: string,
        diagnosticsOptions: DiagnosticsOptions,
        modeConfiguration: ModeConfiguration
    ) {
        this._languageId = languageId
        this.setDiagnosticsOptions(diagnosticsOptions)
        this.setModeConfiguration(modeConfiguration)
    }

    get onDidChange(): IEvent<LanguageServiceDefaults> {
        return this._onDidChange.event
    }

    get languageId(): string {
        return this._languageId
    }

    get modeConfiguration(): ModeConfiguration {
        return this._modeConfiguration
    }

    get diagnosticsOptions(): DiagnosticsOptions {
        return this._diagnosticsOptions
    }

    setDiagnosticsOptions(options: DiagnosticsOptions): void {
        this._diagnosticsOptions = options || Object.create(null)
        this._onDidChange.fire(this)
    }

    setModeConfiguration(modeConfiguration: ModeConfiguration): void {
        this._modeConfiguration = modeConfiguration || Object.create(null)
        this._onDidChange.fire(this)
    }
}

const diagnosticDefault: Required<DiagnosticsOptions> = {
}

const modeConfigurationDefault: Required<ModeConfiguration> = {
    documentFormattingEdits: true,
    documentRangeFormattingEdits: true,
    completionItems: true,
    hovers: true,
    documentSymbols: true,
    colors: true,
    foldingRanges: true,
    diagnostics: true,
    selectionRanges: true,
    links: false,
    documentHighlights: false,
    rename: false
}

export const defaults: LanguageServiceDefaults = new LanguageServiceDefaultsImpl(
    'wgsl',
    diagnosticDefault,
    modeConfigurationDefault
)

languages.register({
    id,
    extensions: ['.wgsl'],
    aliases: ['Wgsl', 'WGSL', 'shader'],
})

languages.onLanguage(id, () => {
    import('./wgslMode').then(mode => mode.setupMode(defaults, id))
    languages.setMonarchTokensProvider(id, monarch)
    languages.setLanguageConfiguration(id, config)
})
