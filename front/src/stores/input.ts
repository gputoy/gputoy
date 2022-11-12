import type { Action } from "src/generated/types"
import { writable } from "svelte/store"

export const wLastInputAction = writable<{ code: string, action?: Action } | null>(null)
