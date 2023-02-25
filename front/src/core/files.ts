import type { File, Files, SupportedExtension } from '$common'
import { wFiles } from '$stores'
import { get, type Writable } from 'svelte/store'

export type FilesExtras = {
	newFile: (file: File) => string
	getFile: (fileid: string) => File | null
	writeFile: (fileid: string, data: string) => void
	updateFileMeta: (fileid: string, meta: Partial<Omit<File, 'data'>>) => void
	removeFile: (fileid: string) => void
}
export function initFilesMethods(files: Writable<Files>): FilesExtras {
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
			if (!map[fileid]) return { map }
			map[fileid] = {
				...map[fileid],
				data
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
}

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
	dir: string
	/**
	 * Path from root
	 */
	absoluteDir: string
	/**
	 * Children of directory.
	 * List entries can either be a file or another FileTreeNode
	 */
	children: FileTreeNodeChild[]
}

/**
 * Retreieves parent absolute path from absoute path
 * @param path a path i.e. '/some/path/to/file.txt'
 * @returns parent path i.e. '/some/path/to'
 */
export function pathParent(path: string): string {
	const [_, ...paths] = path.split('/')
	if (paths.length <= 1) return ''
	paths.pop()
	return '/' + paths.join('/')
}

/**
 * Retrieves file name from path
 * @param path i.e. '/some/path/to/file.txt'
 * @returns [fileName, extension, ...dirs]
 */
export function pathToParts(
	path: string
): [string, string, string[]] | undefined {
	if (!isValidPath(path)) return
	const [file, ...dirs] = path.trim().split('/').reverse()
	const [extension, ...fileName] = file.split('.').reverse()
	return [fileName.join('.'), extension, dirs]
}

/**
 * File name as displayed in frontend
 * @param file
 * @returns file name
 */
export function getCanonicalName(file: string | File | FileWithId): string {
	if (typeof file == 'string') file = wFiles.getFile(file)!
	return file.fileName + '.' + file.extension
}

export function fileWithNewPath(
	file: File | FileWithId,
	newPath: string
): File | undefined {
	const [fileName, extension, dirs] = pathToParts(newPath) ?? []
	if (!fileName) return
	return {
		...file,
		fileName,
		extension: extension as SupportedExtension,
		dir: dirs!.pop() ?? ''
	}
}

const RE_VALID_FILE_ID = /\/([.]?[_]*[\w_-]+\/)*([.]?[_]*[\w_-]*[.][a-z]+)/g
/**
 * Determine whether the path has valid formatting
 * @param path
 * @returns whether the path is vallid
 */
export function isValidPath(path: string): boolean {
	return path.match(RE_VALID_FILE_ID)?.length == 1
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
export function treeFromFiles(files: Files): FileTreeNode {
	const ret: FileTreeNode = { dir: '', absoluteDir: '', children: [] }
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
			let dirIndex = ptr.children
				.filter((entry) => 'dir' in entry)
				.map((entry) => entry.dir)
				.indexOf(path)
			// dir not present, create it
			if (dirIndex < 0) {
				dirIndex =
					ptr.children.push({ dir: path, absoluteDir, children: [] }) - 1
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
		const name1: string = 'dir' in a ? a.dir : getCanonicalName(a as FileWithId)
		const name2: string = 'dir' in b ? b.dir : getCanonicalName(b as FileWithId)
		return name1 > name2 ? 1 : -1
	})
	// Sort the children
	ptr.children.forEach(sortChildren)
}

/**
 * Ensure this potential rename is valid. If return is undefined, then it is valid
 * @param node file node that is being renamed
 * @param rename new name
 * @returns error message | undefined
 */
export function validateRename(
	node: FileTreeNodeChild,
	rename: string
): string | undefined {
	if (rename.length == 0) return 'cannot be empty'
	if ('id' in node) {
		const newId = pathParent(node.id) + '/' + rename.trim()
		if (newId != node.id && wFiles.getFile(newId) != null) return 'exists'
		if (!isValidPath(newId)) return 'invalid'
	} else {
		console.log(node.absoluteDir)
	}
	return
}
