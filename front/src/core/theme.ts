import { browser } from '$app/environment'
import themes from '$gen/themes.json'
import { sealWritable } from '$stores'
import Cookies from 'js-cookie'
import { get, writable, type StartStopNotifier } from 'svelte/store'

export const THEME_COOKIE_NAME = 'theme'

export type Theme = 'light' | 'dark'

const initTheme: StartStopNotifier<Theme> = (set) => {
	if (!browser) return
	const theme = Cookies.get(THEME_COOKIE_NAME)
	if (theme) {
		set(theme as Theme)
	}
}

const wTheme = writable<Theme>('dark', initTheme)
export const rTheme = sealWritable(wTheme, 'workspace.theme')

export function toggleTheme() {
	wTheme.update((theme) => {
		if (!browser) return theme
		const newTheme = theme == 'light' ? 'dark' : 'light'
		document.body.className = newTheme
		Cookies.set(THEME_COOKIE_NAME, newTheme)
		return newTheme
	})
}

type CssVar = keyof (typeof themes)['dark'] | keyof (typeof themes)['light']
export function cssVar(cssVar: CssVar): string {
	const theme = themes[get(wTheme)]
	const cssValue = theme[cssVar]
	return cssValue
}

export function setCssVar(identifier: string, value: string) {
	if (!browser) return
	document.body.style.setProperty(identifier, value)
}
