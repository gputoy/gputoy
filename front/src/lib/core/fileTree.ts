import type { File, Files } from "src/generated/types"

/**
 * File with cached id, so the file tree ui can easily 
 * create action callbacks
 */
export type FileWithId = File & { id: string }
export type FileTreeNodeChild = FileTreeNode | FileWithId

/**
 * Node of file tree
 */
export type FileTreeNode = {
    /**
     * Directory name
     */
    dir: string,
    /**
     * Path from root
     */
    absoluteDir: string,
    /**
     * Children of directory.
     * List entries can either be a file or another FileTreeNode
     */
    children: FileTreeNodeChild[]
}

/**
 * Transforms map representation to tree representation for rendering.
 * 
 * ```
 * {
 *    '/run.json': {...},
 *    '/shaders/main.wgsl': {...},
 * }
 * ```
 *  transforms to
 * ```
 * {
 *     'dir': '',
 *     'children': [
 *         {
 *             'fileid': '/run.json',
 *             ...
 *         },
 *         {
 *             'dir': 'shaders',
 *             'children': [
 *                 {
 *                     'fileid': '/shaders/main.wgsl',
 *                     ...
 *                 }
 *             ]
 *         }
 *     ] 
 * }
 * ```
 * @param files Files from store
 * @returns Tree representation of files 
 */
export function fromFiles(files: Files): FileTreeNode {
    let ret: FileTreeNode = { dir: '', absoluteDir: '', children: [] }
    let ptr = ret

    // add every file to tree in any order
    for (const [fileid, file] of Object.entries(files.map)) {
        const canonicalName = getCanonicalName(file)
        const paths = fileid.slice(1).split('/')
        let absoluteDir = ''

        // find where to insert file, creating directories if needed
        for (const path of paths) {
            absoluteDir += '/' + path

            // found location of file
            if (path === canonicalName) {
                ptr.children.push({ ...file, id: fileid })
                break
            }
            // else try to find directory
            let dirIndex = ptr.children.filter(entry => 'dir' in entry).map(entry => entry.dir).indexOf(path)
            // dir not present, create it
            if (dirIndex < 0) {
                dirIndex = ptr.children.push({ dir: path, absoluteDir, children: [] }) - 1
            }
            // Move down the tree
            ptr = ptr.children[dirIndex] as FileTreeNode
        }
        // Reset pointer to root of tree
        ptr = ret
    }

    // walk through tree to sort children of each directory by alphabetical order
    sortChildren(ret)
    return ret
}

/**
 * Sorts file tree in place based on alpabetical order of directory name or canonical file name 
 * @param ptr Root of file tree
 * @returns void 
 */
function sortChildren(ptr: FileTreeNodeChild) {
    // Base case: is a file so nothing to sort
    if (!('children' in ptr)) return
    // Sort this directory's list of children 
    ptr.children.sort((a, b) => {
        let name1: string = 'dir' in a ? a.dir : getCanonicalName(a as FileWithId)
        let name2: string = 'dir' in b ? b.dir : getCanonicalName(b as FileWithId)
        return name1 > name2 ? 1 : -1
    })
    // Sort the children 
    ptr.children.forEach(sortChildren)
}

/**
 * File name as displayed in frontend
 * @param file 
 * @returns file name 
 */
export function getCanonicalName(file: File | FileWithId): string {
    return file.fileName + '.' + file.extension
}