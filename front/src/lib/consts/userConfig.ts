import type { Keybinds } from '$lib/core/input'
import type { UserEditorConfig, UserGeneralConfig } from 'src/generated/types'

export const DEFAULT_USER_GENERAL_CONFIG: UserGeneralConfig = {
    projectPanelSize: 12,
    editorPanelSize: 50,
    resourcePanelSize: 40,
} as const

export const DEFAULT_USER_EDITOR_CONFIG: UserEditorConfig = {
    lineNumbers: 'on',
    fontFamily: 'mono',
    fontSize: 12,
    vimMode: false,
    minimap: false,
} as const

export const DEFAULT_USER_KEYBINDS: Keybinds = {

    'C-g': {
        action: { ty: 'toggleConsole' }
    },
    'C-q': {
        action: {
            ty: 'togglePanel',
            c: 'projectPanel'
        }
    },
    'C-e': {
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
    },
    'C-S-d': {
        action: {
            ty: 'toggleDebugPanel'
        }
    },
    'C-s': {
        action: {
            ty: 'saveAllFiles'
        }
    },
    'C-u': {
        action: {
            ty: 'closeFile'
        }
    },
    'C-S-f': {
        action: {
            ty: 'fork'
        }
    },
    'C-S-g': {
        action: {
            ty: 'publish'
        },
        condition: 'userLoggedIn'
    },


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
            description: 'Default width of the project panel.',
            units: '%',
            min: 10,
            max: 50,
        },
        editorPanelSize: {
            type: 'number',
            description: 'Default width of the editor panel.',
            units: '%',
            min: 10,
            max: 90,
        },
        resourcePanelSize: {
            type: 'number',
            description: 'Default height for the resource panel',
            units: '%',
            min: 10,
            max: 90,
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
            options: ['on', 'interval', 'relative', 'off'] as Array<string>,
        },
        vimMode: {
            type: 'toggle',
            description: 'Enable vim movements in code editor.',
        },
        minimap: {
            type: "toggle",
            description: 'Enable minimap in top right of editor that shows an overview of file.'
        }
    }
} as const

export const GENERAL_CONFIG_KEYS: readonly GeneralConfigKey[] = [
    'projectPanelSize',
    'editorPanelSize',
    'resourcePanelSize'
] as const

export const EDITOR_CONFIG_KEYS: readonly EditorConfigKey[] = [
    'fontFamily',
    'fontSize',
    'lineNumbers',
    'vimMode',
    'minimap'
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