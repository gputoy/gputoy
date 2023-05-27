import { browser } from '$app/environment'
import { wTheme } from '$stores'
import Cookies from 'js-cookie'

export type Theme = 'light' | 'dark'

export function toggleTheme() {
	wTheme.update((theme) => {
		if (!browser) return theme
		const newTheme = theme == 'light' ? 'dark' : 'light'
		document.body.className = newTheme
		Cookies.set('t', newTheme)
		return newTheme
	})
}

export function cssVar(identifier: string): string {
	if (!browser) return ''
	return getComputedStyle(document.body).getPropertyValue(identifier).trim()
}

export function setCssVar(identifier: string, value: string) {
	if (!browser) return
	document.body.style.setProperty(identifier, value)
}

export function delayedFocus(el: HTMLElement, wait: number) {
	setTimeout(() => {
		el.focus({ preventScroll: true })
	}, wait)
}
