import { DEFAULT_FILES } from "$lib/consts/project"
import type { File, Files } from "src/generated/types"
import { get } from "svelte/store"
import { makeEnhanced } from "../enhanced"

export type FilesExtras = {
    newFile: (file: File) => string
    getFile: (fileid: string) => File | null
    writeFile: (fileid: string, data: string) => void
    updateFileMeta: (fileid: string, meta: Partial<Omit<File, 'data'>>) => void
    removeFile: (fileid: string) => void
}
export default makeEnhanced<Files, FilesExtras>(DEFAULT_FILES, function (files) {

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

    return { newFile, getFile, writeFile, updateFileMeta, removeFile }
}) 