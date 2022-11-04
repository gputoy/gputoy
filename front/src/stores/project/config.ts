import { DEFAULT_CONFIG } from "$lib/consts/project"
import type { Config } from "src/generated/types"
import { writable, type Writable } from "svelte/store"

export type WritableConfig = Writable<Config> & {

}
export default function makeConfig(): WritableConfig {
    const config = writable<Config>(DEFAULT_CONFIG)

    return { ...config }
}