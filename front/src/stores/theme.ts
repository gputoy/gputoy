import { browser } from "$app/environment"
import Cookies from 'js-cookie'
import { writable } from "svelte/store"

export type Theme = "light" | "dark"
export const theme = writable<Theme>("dark", (set) => {
  if (!browser) return
  const theme = Cookies.get('t')
  if (theme) {
    set(theme as Theme)
  }
})
export function toggle() {
  theme.update((theme) => {
    if (!browser) return theme
    const newTheme = theme == 'light' ? 'dark' : 'light'
    document.body.className = newTheme
    Cookies.set('t', newTheme)
    return newTheme
  })
}