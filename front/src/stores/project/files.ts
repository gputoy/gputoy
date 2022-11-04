import { DEFAULT_FILES } from "$lib/consts/project"
import type { File, Files } from "src/generated/types"
import { get, writable, type Writable } from "svelte/store"

export type WritableFiles = Writable<Files> & {
    newFile: (file: File) => string
    getFile: (fileid: string) => File | null
    writeFile: (fileid: string, data: string) => void
    updateFileMeta: (fileid: string, meta: Partial<Omit<File, 'data'>>) => void
    removeFile: (fileid: string) => void
}
export default function makeFiles(): WritableFiles {
    let files = writable(DEFAULT_FILES)

    function newFile(file: File): string {
        const fileid = `${file.dir}/${file.fileName}.${file.extension}`
        files.update(({ map }) => {
            map[fileid] = file
            return { map }
        })
        return fileid
    }

    function getFile(fileid: string): File | null {
        return get(files).map[fileid] ?? null
    }

    function writeFile(fileid: string, data: string) {
        files.update(({ map }) => {
            map[fileid] = {
                ...map[fileid],
                data,
            }
            return { map }
        })
    }

    function updateFileMeta(fileid: string, meta: Partial<Omit<File, 'data'>>) {
        const file = getFile(fileid)
        if (!file) return
        removeFile(fileid)
        newFile({ ...file, ...meta })
    }

    function removeFile(fileid: string) {
        files.update(({ map }) => {
            delete map[fileid]
            return { map }
        })
    }

    return { ...files, newFile, getFile, writeFile, updateFileMeta, removeFile }
}