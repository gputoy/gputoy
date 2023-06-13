import {
	CompletionIndex,
	COMPLETION_KEY_TO_INDEX,
	type ArgDescriptor,
	type CompletionEntry,
	type CompletionKey
} from '$gen'
import { getSealedStoreKeys, sealWritable, wFiles } from '$stores'
import * as wasm from '$wasm/common'

import STATIC_COMPLETIONS from '$gen/workspace/completions.json'
import { get, writable } from 'svelte/store'
import { handleClientError } from './console'
import { searchSplit } from './util'

const wCompletions = writable<Completions>({ matches: [] })
const wCompletionIndex = writable(-1)
const wCompletionsPosition = writable<CompletionsPosition | null>(null)

export const rCompletions = sealWritable(wCompletions, 'completions.matches')
export const rCompletionIndex = sealWritable(
	wCompletionIndex,
	'completions.index'
)
export const rCompletionsPosition = sealWritable(
	wCompletionsPosition,
	'completions.position'
)

export function reset() {
	wCompletions.set({ matches: [] })
	wCompletionIndex.set(-1)
	wCompletionsPosition.set(null)
}

/**
 * Regnerate completions, writing to stores that will trigger an update in CompletionsProvider.svelte
 *
 * If no `argDescriptor` is supplied, it will intepret `value` as an action.
 * Else, it will use the `argDescriptor.completionKey` to determine which completion to generate.
 * @param value string to complete
 * @param cursorIndex where the input cursor lies in the string
 * @param argDescriptor An optional argument descriptor.
 */
export function regenerateCompletions(
	value: string,
	cursorIndex: number,
	argDescriptor?: ArgDescriptor
) {
	if (!argDescriptor) {
		const completionInfo = wasm.completion(value, cursorIndex)
		if ('severity' in completionInfo) {
			handleClientError(completionInfo)
			return
		}
		if (completionInfo.cursor_word_index == null) return reset()
		argDescriptor =
			completionInfo.arg_descriptors[completionInfo.cursor_word_index]
		if (!argDescriptor) return reset()
	} else {
		argDescriptor = { ...argDescriptor, value }
	}
	const completionResult = generateCompletions(argDescriptor)
	wCompletionIndex.update((pos) =>
		Math.min(completionResult.matches.length - 1, pos)
	)
	wCompletions.set(completionResult)
}

/**
 * Shifts the completion index by the given amount and returns the completions match
 * at the new location.
 * @param shift The amount to shift by, positive or negative.
 * @returns
 */
export function shiftCompletionIndex(
	shift: number
): CompletionEntryMatch | null {
	const completions = get(rCompletions)
	const len = completions.matches.length
	let newIndex = -1
	wCompletionIndex.update((index) => {
		newIndex = index + shift
		if (newIndex < 0) newIndex = Math.max(-1, newIndex + len)
		newIndex %= len
		return newIndex
	})
	return completions.matches[newIndex] ?? null
}

export function setCompletionIndex(index: number) {
	const completions = get(wCompletions)
	wCompletionIndex.set(
		Math.min(completions.matches.length, Math.max(index, -1))
	)
}

export function updateLocationFromDomRect(rect: DOMRect) {
	wCompletionsPosition.set(rect)
}

export type CompletionEntryMatch = Omit<CompletionEntry, 'matches'> & {
	matchParts: [string, string, string]
	alias: boolean
	insertText: string
}

export type Completions = {
	matches: CompletionEntryMatch[]
	argInfo?: ArgInfo
}

export type CompletionsPosition = DOMRect

export type ArgInfo = {
	name: string
	description: string
	ty: CompletionKey
}

// TODO: should be memo'd (or derive store'd)
function generatePathCompletions(): CompletionEntry[] {
	const paths = wFiles.paths()
	return paths.map((path) => ({
		matches: [path],
		snippetText: path,
		description: 'path'
	}))
}

// TODO: should be memo'd (or derive store'd)
function generateFilePathCompletions(): CompletionEntry[] {
	const paths = wFiles.filePaths()
	return paths.map((path) => {
		let file = wFiles.getFile(path)!
		return {
			matches: [path],
			snippetText: path,
			description: file.extension + ' file'
		}
	})
}

var _storeKeyCompletions: CompletionEntry[]
// This only needs to be done once, but only AFTER all the stores have been
// sealed. Best solultion is to generate at the first request
function generateStoreKeyCompletions(): CompletionEntry[] {
	if (_storeKeyCompletions === undefined) {
		_storeKeyCompletions = getSealedStoreKeys().map((key) => {
			return {
				matches: ['$' + key],
				snippetText: 'store',
				description: ''
			}
		})
	}
	return _storeKeyCompletions
}

export function generateCompletions(argDescriptor: ArgDescriptor): Completions {
	const { name, value, completionKey, description } = argDescriptor
	const completionIndex = COMPLETION_KEY_TO_INDEX[completionKey]
	let completions = STATIC_COMPLETIONS[completionIndex] as CompletionEntry[]
	const dynamicCompletions = generateDynamicCompletions(completionIndex)
	const filteredCompletions = completions
		.concat(dynamicCompletions)
		.map((completion) => {
			const insertText = completion.matches[0]

			const match = completion.matches.find((match) => match.includes(value))
			if (!match) return null

			const matchParts = searchSplit(match, value)
			if (!matchParts) return null

			return {
				...completion,
				matchParts,
				insertText,
				alias: !(match == insertText)
			}
		})
		.filter((completion) => !!completion) as CompletionEntryMatch[]
	return {
		matches: filteredCompletions,
		argInfo: {
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
		case CompletionIndex.StoreKey:
			return generateStoreKeyCompletions()
		default:
			return []
	}
}
