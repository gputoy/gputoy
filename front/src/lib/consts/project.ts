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
        "/run.json": {
            "data": "...",
            "dir": "",
            "fileName": "run",
            "extension": "json"
        }
    }
} as const