import { browser } from "$app/environment";
import { writable } from "svelte/store";

export type Theme = "light" | "dark";
export const theme = writable<Theme>("light", (set) => {
  if (!browser) return;
  const localTheme = localStorage.getItem('theme') as Theme ?? 'light'
  set(localTheme);
  if (localTheme == 'dark') document.body.classList.add('dark-theme');
  else document.body.classList.remove('dark-theme');
});
export function toggle() {
  theme.update((th) => {
    const newth = th == 'light' ? 'dark' : 'light';
    if (browser) {
      localStorage.setItem('theme', newth);
    }
    if (newth == 'dark') document.body.classList.add('dark-theme');
    else document.body.classList.remove('dark-theme');
    return newth;
  });


}