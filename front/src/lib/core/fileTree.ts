import type { File, Files } from "src/generated/types"

export type FileWithId = File & { id: string }
export type FileTreeNodeChild = FileTreeNode | FileWithId
export type FileTreeNode = {
    dir: string,
    children: FileTreeNodeChild[]
}

export function fromFiles(files: Files): FileTreeNode {
    let ret: FileTreeNode = { dir: '', children: [] }
    let ptr = ret

    // add every file to tree in any order
    for (const [fileid, file] of Object.entries(files.map)) {
        const canonicalName = getCanonicalName(file)
        const paths = fileid.slice(1).split('/')

        // find where to insert file, creating directories if needed
        for (const path of paths) {
            // found location of file
            if (path === canonicalName) {
                ptr.children.push({ ...file, id: fileid })
                break
            }
            // else try to find directory
            let dirIndex = ptr.children.filter(entry => 'dir' in entry).map(entry => entry.dir).indexOf(path)
            // dir not present, create it
            if (dirIndex < 0) {
                dirIndex = ptr.children.push({ dir: path, children: [] }) - 1
            }
            ptr = ptr.children[dirIndex] as FileTreeNode
        }
        ptr = ret
    }

    // walk through tree to sort children of each directory by alphabetical order
    sortChildren(ret)
    return ret
}

function sortChildren(ptr: FileTreeNodeChild) {
    if (!('children' in ptr)) return
    ptr.children.sort((a, b) => {
        let name1: string = 'dir' in a ? a.dir : getCanonicalName(a as FileWithId)
        let name2: string = 'dir' in b ? b.dir : getCanonicalName(b as FileWithId)
        return name1 > name2 ? 1 : -1
    })
    ptr.children.forEach(sortChildren)
}

export function getCanonicalName(file: File | FileWithId): string {
    return file.fileName + '.' + file.extension
}