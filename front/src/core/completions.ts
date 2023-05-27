import {
	CompletionIndex,
	COMPLETION_KEY_TO_INDEX,
	type CompletionEntry,
	type CompletionInfo,
	type CompletionKey
} from '$gen'
import { wFiles } from '$stores'

import Folder from '~icons/codicon/folder'
import Terminal from '~icons/codicon/terminal'

import STATIC_COMPLETIONS from '$gen/workspace/completions.json'

export type Completions = {
	completions: CompletionEntry[]
	arg?: ArgInfo
}

export const DEFAULT_ICON = Folder
export const ICONS = [DEFAULT_ICON, Terminal, Folder]
export type ArgInfo = {
	name: string
	description: string
	ty: CompletionKey
}

function generatePathCompletions(): CompletionEntry[] {
	const paths = wFiles.paths()
	return paths.map((path) => ({
		insertText: path,
		name: path,
		description: 'path'
	}))
}

function generateFilePathCompletions(): CompletionEntry[] {
	const paths = wFiles.filePaths()
	return paths.map((path) => {
		let file = wFiles.getFile(path)!
		return {
			name: path,
			insertText: path,
			description: file.extension + ' file'
		}
	})
}

// export function filteredBy(completions: CompletionList, filter: string) {
//     if (filter.length == 0) return completions
//     let { index, list } = completions
//     list = list.filter((c) => getCompletionInsertText(c).indexOf(filter) >= 0)
//     return {
//         len: list.length,
//         index: Math.min(index, list.length - 1),
//         list,
//     }
// }

export function generateCompletions(
	completionInfo: CompletionInfo
): Completions {
	const index = completionInfo.cursor_word_index
	const argDescriptor = completionInfo.arg_descriptors[index]
	if (!argDescriptor) {
		return {
			completions: []
		}
	}
	const { name, value, completionKey, description } = argDescriptor
	const completionIndex = COMPLETION_KEY_TO_INDEX[completionKey]
	console.log('index', completionIndex, STATIC_COMPLETIONS[completionIndex])
	let completions = STATIC_COMPLETIONS[completionIndex] as CompletionEntry[]
	const dynamicCompletions = generateDynamicCompletions(completionIndex)
	console.log('dyn completions', dynamicCompletions)
	completions.concat(dynamicCompletions)
	return {
		completions,
		arg: {
			name,
			description,
			ty: completionKey
		}
	}
}

export function generateDynamicCompletions(
	completionIndex: CompletionIndex
): CompletionEntry[] {
	switch (completionIndex) {
		case CompletionIndex.FilePath:
			return generateFilePathCompletions()
		case CompletionIndex.Path:
			return generatePathCompletions()
		case CompletionIndex.Resource:
			return []
		default:
			return []
	}
}
