// @TODO: move icon logic to more appropriate module
import Resource from '~icons/codicon/database'
import File from '~icons/codicon/file'
import Folder from '~icons/codicon/folder'
import Terminal from '~icons/codicon/terminal'
import Display from '~icons/material-symbols/desktop-windows-outline'
import Dev from '~icons/material-symbols/developer-board-outline'
import Keyboard from '~icons/material-symbols/keyboard-rounded'
import Gear from '~icons/material-symbols/settings-rounded'

export default {
	Resource,
	Gear,
	Keyboard,
	Dev,
	Terminal,
	Display,
	Folder,
	File
}

export const DEFAULT_ICON = Gear
export const COMPLETION_ICONS = [
	DEFAULT_ICON, // Empty
	Terminal, // ActionKey
	DEFAULT_ICON, // Str
	File, // FilePath,
	Folder, // Path,
	Resource, // Resource,
	Gear, // PreferenceKey,
	Display, // Region,
	Keyboard, // Key,
	Dev // StoreKey
]
