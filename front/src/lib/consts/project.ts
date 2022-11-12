import type { Config, Files, Layout } from "src/generated/types"

/**
 * STATIC DEFAULTS
 */
export const DEFAULT_LAYOUT: Layout = {
    isStatusOpen: true,
    fileIndex: 0,
    workspace: ["/shaders/main.wgsl", "/run.json"] as string[],
    projectPanel: {
        show: true,
        size: 20,
    },
    editorPanel: {
        show: true,
        size: 50,
    },
    resourcePanel: {
        show: true,
        size: 50,
    }

} as const
export const DEFAULT_CONFIG: Config = {} as const
export const DEFAULT_FILES: Files = {
    map: {
        "/shaders/main.wgsl": {
            "data": "...",
            "dir": "shaders",
            "fileName": "main",
            "extension": "wgsl",
        },
        "/shaders/types.wgsl": {
            "data": "...",
            "dir": "shaders",
            "fileName": "types",
            "extension": "wgsl",
        },

        "/shaders/extras/component.wgsl": {
            "data": "...",
            "dir": "extras",
            "fileName": "component",
            "extension": "wgsl",
        },
        "/Readme.md": {
            "data": "Welcome to this project!",
            "dir": "",
            "fileName": "Readme",
            "extension": "md",
        },
        "/run.json": {
            "data": "...",
            "dir": "",
            "fileName": "run",
            "extension": "json"
        }
    }
} as const