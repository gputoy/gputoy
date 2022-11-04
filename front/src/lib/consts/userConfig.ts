import type { FilteredAction, UserEditorConfig, UserGeneralConfig } from 'src/generated/types'

export const DEFAULT_USER_GENERAL_CONFIG: UserGeneralConfig = {
    projectPanelSize: 200
} as const

export const DEFAULT_USER_EDITOR_CONFIG: UserEditorConfig = {
    lineNumbers: 'normal',
    fontFamily: 'mono',
    fontSize: 12,
} as const

export const DEFAULT_USER_KEYBINDS: { [key: string]: FilteredAction } = {

    'C-g': {
        action: { ty: 'toggleConsole' }
    },
    'C-q': {
        action: {
            ty: 'togglePanel',
            c: 'projectPanel'
        }
    },
    'C-y': {
        action: {
            ty: 'togglePanel',
            c: 'editorPanel'
        }
    },
    'C-r': {
        action: {
            ty: 'togglePanel',
            c: 'resourcePanel'
        }
    },
    'C-j': {
        action: {
            ty: 'previousDocument'
        }
    },
    'C-k': {
        action: {
            ty: 'nextDocument'
        }
    }

} as const

/**
 * 
 * Metadata for auto-generating user config ui
 * Will need upkeep work to stay in sync with gpu-common types
 */
export const USER_CONFIG_META: ConfigMeta = {
    general: {
        projectPanelSize: {
            type: 'number',
            description: 'How wide the project info pane is.',
            units: 'px',
            min: 100,
            max: 400,
        },
    },
    editor: {
        fontSize: {
            type: 'number',
            description: 'Font size in code editor',
            units: 'px',
            min: 5,
            max: 32,
        },
        fontFamily: {
            type: 'text',
            description: 'Font in code editor.',
        },
        lineNumbers: {
            type: 'select',
            description: 'How the code editor will display line numbers.',
            options: ['normal', 'relative', 'off'] as Array<string>,
        }
    }
} as const

export const GENERAL_CONFIG_KEYS: readonly GeneralConfigKey[] = [
    'projectPanelSize'
] as const

export const EDITOR_CONFIG_KEYS: readonly EditorConfigKey[] = [
    'fontFamily',
    'fontSize',
    'lineNumbers'
] as const

export type ConfigScope = 'general' | 'editor'

export type ConfigMeta = {
    readonly general: ConfigCategory<GeneralConfigKey>,
    readonly editor: ConfigCategory<EditorConfigKey>,
}
export type GeneralConfigKey = keyof UserGeneralConfig
export type EditorConfigKey = keyof UserEditorConfig
export type ConfigKey = GeneralConfigKey | EditorConfigKey
export type ConfigCategory<K extends string | number | symbol> = {
    [key in K]: ConfigItemMeta
}

export type ConfigItemMeta = {
    readonly type: string,
    readonly description?: string,
    readonly units?: string,
    readonly min?: number,
    readonly max?: number,
    readonly regex?: RegExp,
    readonly options?: Array<string>,
}