import type { FilteredAction } from "src/generated/types"

export const MENUKEYS = ['file', 'edit', 'project', 'view', 'help'] as const
export type MenuKey = typeof MENUKEYS[number]

/**
 * Contains all actions for navbar menu on /dev page
 */
export const MENU_MAP: Record<MenuKey, ReadonlyArray<ReadonlyArray<MenuEntry>>> = {
    'file': [
        [
            {
                name: 'New Project',
                fAction: {
                    action: {
                        'ty': 'createNewProject'
                    }
                }
            },
            {
                name: 'New File',
                fAction: {
                    action: {
                        'ty': 'createNewFile'
                    }
                }
            }
        ],
        [
            {
                name: 'Save to remote',
                fAction: {
                    action: {
                        'ty': 'saveProjectToRemote'
                    },
                    condition: 'userLoggedIn',
                }
            },
            {
                name: 'Save as',
                fAction: {
                    action: {
                        'ty': 'saveAllFiles'
                    },
                    condition: 'filesDirty',
                }
            },
            {
                name: 'Save all',
                fAction: {
                    action: {
                        'ty': 'saveCurrentFile',
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
                        'ty': 'fork'
                    }
                }
            },
            {
                name: 'Publish',
                fAction: {
                    action: {
                        'ty': 'publish'
                    },
                    condition: 'userLoggedIn'
                }
            },
        ],
        [
            {
                name: 'Preferences',
                fAction: {
                    action: {
                        'ty': 'toggleUserPreferences'
                    }
                }
            }
        ],
        [
            {
                name: 'Close file',
                fAction: {
                    action: {
                        'ty': 'closeFile'
                    }
                }
            },

            {
                name: 'Exit',
                fAction: {
                    action: {
                        'ty': 'closeProject'
                    }
                }
            }
        ],
    ],
    'edit': [],
    'project': [],
    'view': [
        [
            {
                name: 'Toggle Project Panel',
                fAction: {
                    action: {
                        'ty': 'togglePanel',
                        'c': 'projectPanel'
                    }
                }
            },
            {
                name: 'Toggle Editor Panel',
                fAction: {
                    action: {
                        'ty': 'togglePanel',
                        'c': 'editorPanel'
                    }
                }
            },
            {
                name: 'Toggle Resource Panel',
                fAction: {
                    action: {
                        'ty': 'togglePanel',
                        'c': 'resourcePanel'
                    }
                }
            },
        ]
    ],
    'help': []
} as const

export type MenuEntry = {
    name: string,
    fAction?: FilteredAction,
}

