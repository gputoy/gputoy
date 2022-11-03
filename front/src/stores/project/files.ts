import type { File, Files } from "src/generated/types"
import { get, writable, type Writable } from "svelte/store"

export const DEFAULT_FILES: Files = {
    map: {
        "shaders/main.wgsl": {
            "data": "...",
            "dir": "shaders",
            "fileName": "main",
            "extension": "wgsl",
        },
        "run.json": {
            "data": "...",
            "dir": "",
            "fileName": "run",
            "extension": "json"
        }
    }
} as const

export type WritableFiles = Writable<Files> & {
    getFile: (fileid: string) => File | null
    writeFile: (fileid: string, data: string) => void
    removeFile: (fileid: string) => void
}
export default function makeFiles(): WritableFiles {
    let $files = writable(DEFAULT_FILES)

    function getFile(fileid: string): File | null {
        return get($files).map[fileid] ?? null
    }

    function writeFile(fileid: string, data: string) {
        $files.update(files => {
            const map = files.map
            map[fileid] = {
                ...map[fileid],
                data,
            }
            return { map }
        })
    }

    function removeFile(fileid: string) {

    }

    return { ...$files, getFile, writeFile, removeFile }
}