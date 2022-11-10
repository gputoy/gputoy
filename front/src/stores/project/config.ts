import { DEFAULT_CONFIG } from "$lib/consts/project"
import type { Config } from "src/generated/types"
import { makeEnhanced } from "../enhanced"

export type ConfigExtras = {
}
export default makeEnhanced<Config, ConfigExtras>(DEFAULT_CONFIG, function (config) {
    return {}
}) 