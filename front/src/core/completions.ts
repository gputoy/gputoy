import {
    CompletionIndex,
    COMPLETION_KEY_TO_INDEX,
    type CompletionEntry,
    type CompletionInfo,
    type CompletionKey
} from '$gen'
import { wFiles } from '$stores'

import Resource from '~icons/codicon/database'
import File from '~icons/codicon/file'
import Folder from '~icons/codicon/folder'
import Terminal from '~icons/codicon/terminal'
import Display from '~icons/material-symbols/desktop-windows-outline'
import Keyboard from '~icons/material-symbols/keyboard-rounded'
import Settings from '~icons/material-symbols/settings-rounded'

import STATIC_COMPLETIONS from '$gen/workspace/completions.json'

export type Completions = {
    completions: CompletionEntry[]
    arg?: ArgInfo
}

export const DEFAULT_ICON = Settings
export const ICONS = [
    DEFAULT_ICON,       // Empty 
    Terminal,           // ActionKey 
    DEFAULT_ICON,       // Str
    File,	            // FilePath,
    Folder,             // Path,
    Resource,           // Resource,
    Settings,           // PreferenceKey,
    Display,            // Region,
    Keyboard,           // Key,
]
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
    let completions = STATIC_COMPLETIONS[completionIndex] as CompletionEntry[]
    const dynamicCompletions = generateDynamicCompletions(completionIndex)
    return {
        completions: completions
            .concat(dynamicCompletions)
            .filter(completion => completion.insertText.includes(value)),
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
