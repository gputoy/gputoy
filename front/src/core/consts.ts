import type {
	Config,
	DirNodeState,
	Files,
	FilteredAction,
	Layout,
	UserEditorPrefs,
	UserGeneralPrefs
} from '$common'
import type { Keybinds } from '$core/input'
import type { RunState } from './runstate'

/**
 *              Points to gpu-front (this)
 */
export const BASE_URL = import.meta.env.VITE_FE_URL + '/'

/**
 *              Points to gpu-back
 */
export const API_URL = import.meta.env.VITE_API_URL + '/'

export const WASM_ANALYZER_URL = BASE_URL + import.meta.env.VITE_MAKE_ANALYZER_PATH
export const WASM_CLIENT_URL = BASE_URL + import.meta.env.VITE_MAKE_CLIENT_PATH

export const DEFAULT_RUN_STATE: RunState = {
	playing: false
}

/**
 *              Project defaults
 */
export const DEFAULT_DIR_NODE_STATE: DirNodeState = {
	open: false,
	isRenaming: false
}
export const DEFAULT_LAYOUT: Layout = {
	isStatusOpen: true,
	fileIndex: 0,
	workspace: ['/shaders/main.wgsl', '/run.json'] as string[],
	projectPanel: {
		show: true,
		size: 20
	},
	editorPanel: {
		show: true,
		size: 50
	},
	resourcePanel: {
		show: true,
		size: 50
	},
	fileTreeState: {
		'/shaders': {
			...DEFAULT_DIR_NODE_STATE,
			open: true
		}
	},
	accordianOpen: {
		Summary: true,
		Files: true
	}
} as const
export const DEFAULT_CONFIG: Config = {
	runner: '/run.json',
	logLevel: 'Info'
} as const
export const DEFAULT_FILES: Files = {
	map: {
		'/shaders/main.wgsl': {
			data: '...',
			dir: 'shaders',
			fileName: 'main',
			extension: 'wgsl'
		},
		'/shaders/types.wgsl': {
			data: '...',
			dir: 'shaders',
			fileName: 'types',
			extension: 'wgsl'
		},

		'/shaders/extras/component.wgsl': {
			data: '...',
			dir: 'extras',
			fileName: 'component',
			extension: 'wgsl'
		},
		'/Readme.md': {
			data: 'Welcome to this project!',
			dir: '',
			fileName: 'Readme',
			extension: 'md'
		},
		'/run.json': {
			data: '...',
			dir: '',
			fileName: 'run',
			extension: 'json'
		}
	}
} as const

/**
 *                  User config defaults
 */
export const DEFAULT_USER_GENERAL_PREFS: UserGeneralPrefs = {
	projectPanelSize: 12,
	editorPanelSize: 50,
	resourcePanelSize: 40,
	consoleWrap: true
} as const

export const DEFAULT_USER_EDITOR_PREFS: UserEditorPrefs = {
	lineNumbers: 'on',
	fontFamily: 'mono',
	fontSize: 12,
	vimMode: false,
	minimap: false
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
			ty: 'saveCurrentFile'
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
	'A-t': {
		action: {
			ty: 'toggleConsole'
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
			description: 'Default width of the project panel.',
			units: '%',
			min: 10,
			max: 50
		},
		editorPanelSize: {
			type: 'number',
			description: 'Default width of the editor panel.',
			units: '%',
			min: 10,
			max: 90
		},
		resourcePanelSize: {
			type: 'number',
			description: 'Default height for the resource panel',
			units: '%',
			min: 10,
			max: 90
		},
		consoleWrap: {
			type: 'toggle',
			description: 'Enable text wrap in the console.'
		}
	},
	editor: {
		fontSize: {
			type: 'number',
			description: 'Font size in code editor',
			units: 'px',
			min: 5,
			max: 32
		},
		fontFamily: {
			type: 'text',
			description: 'Font in code editor.'
		},
		lineNumbers: {
			type: 'select',
			description: 'How the code editor will display line numbers.',
			options: ['on', 'interval', 'relative', 'off'] as Array<string>
		},
		vimMode: {
			type: 'toggle',
			description: 'Enable vim movements in code editor.'
		},
		minimap: {
			type: 'toggle',
			description:
				'Enable minimap in top right of editor that shows an overview of file.'
		}
	}
} as const

export const GENERAL_CONFIG_KEYS: readonly GeneralConfigKey[] = [
	'projectPanelSize',
	'editorPanelSize',
	'resourcePanelSize',
	'consoleWrap'
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
	readonly general: ConfigCategory<GeneralConfigKey>
	readonly editor: ConfigCategory<EditorConfigKey>
}
export type GeneralConfigKey = keyof UserGeneralPrefs
export type EditorConfigKey = keyof UserEditorPrefs
export type ConfigKey = GeneralConfigKey | EditorConfigKey
export type ConfigCategory<K extends string | number | symbol> = {
	[key in K]: ConfigItemMeta
}

export type ConfigItemMeta = {
	readonly type: string
	readonly description?: string
	readonly units?: string
	readonly min?: number
	readonly max?: number
	readonly regex?: RegExp
	readonly options?: Array<string>
}

/**
 *                  Nav UI Menu
 */
export const MENUKEYS = ['file', 'edit', 'project', 'view', 'help'] as const
export type MenuKey = typeof MENUKEYS[number]
export type MenuEntry = {
	name: string
	fAction?: FilteredAction
}

export const MENU_MAP: Record<
	MenuKey,
	ReadonlyArray<ReadonlyArray<MenuEntry>>
> = {
	file: [
		[
			{
				name: 'New Project',
				fAction: {
					action: {
						ty: 'createNewProject'
					}
				}
			},
			{
				name: 'New File',
				fAction: {
					action: {
						ty: 'createNewFile'
					}
				}
			}
		],
		[
			{
				name: 'Save to remote',
				fAction: {
					action: {
						ty: 'saveProjectToRemote'
					},
					condition: 'userLoggedIn'
				}
			},
			{
				name: 'Save as',
				fAction: {
					action: {
						ty: 'saveAllFiles'
					},
					condition: 'filesDirty'
				}
			},
			{
				name: 'Save all',
				fAction: {
					action: {
						ty: 'saveCurrentFile'
					},
					condition: 'currentFileDirty'
				}
			}
		],
		[
			{
				name: 'Fork',
				fAction: {
					action: {
						ty: 'fork'
					}
				}
			},
			{
				name: 'Publish',
				fAction: {
					action: {
						ty: 'publish'
					},
					condition: 'userLoggedIn'
				}
			}
		],
		[
			{
				name: 'Preferences',
				fAction: {
					action: {
						ty: 'toggleUserPreferences'
					}
				}
			}
		],
		[
			{
				name: 'Close file',
				fAction: {
					action: {
						ty: 'closeFile'
					}
				}
			},

			{
				name: 'Exit',
				fAction: {
					action: {
						ty: 'closeProject'
					}
				}
			}
		]
	],
	edit: [],
	project: [
		[
			{
				name: 'Build',
				fAction: {
					action: {
						ty: 'rebuild'
					}
				}
			},
			{
				name: 'Introspect',
				fAction: {
					action: {
						ty: 'toggleConsole'
					}
				}
			}
		]
	],
	view: [
		[
			{
				name: 'Toggle Project Panel',
				fAction: {
					action: {
						ty: 'togglePanel',
						c: 'projectPanel'
					}
				}
			},
			{
				name: 'Toggle Editor Panel',
				fAction: {
					action: {
						ty: 'togglePanel',
						c: 'editorPanel'
					}
				}
			},
			{
				name: 'Toggle Resource Panel',
				fAction: {
					action: {
						ty: 'togglePanel',
						c: 'resourcePanel'
					}
				}
			}
		],
		[
			{
				name: 'Toggle Console',
				fAction: {
					action: {
						ty: 'toggleConsole'
					}
				}
			},
			{
				name: 'Toggle User Preferences',
				fAction: {
					action: {
						ty: 'toggleUserPreferences'
					}
				}
			},
			{
				name: 'Toggle Debug Panel',
				fAction: {
					action: {
						ty: 'toggleDebugPanel'
					}
				}
			}
		],
		[
			// {
			//     name: 'Toggle Dark Mode',
			//     fAction: {
			//         action: {
			//             'ty': 'toggleDarkMode'
			//         }
			//     }
			// }
		]
	],
	help: []
} as const
