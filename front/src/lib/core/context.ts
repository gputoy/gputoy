import { browser } from '$app/environment'
import type { Project } from 'src/generated/types'

import { Context, default as init_module } from '../../../pkg/gpu_wasm'

var context: Context | undefined = undefined

export async function init() {
  if (!browser || !("gpu" in navigator)) return
  await init_module()
  context = await new Context()
  console.log("js:context:init", context)
}

export async function build(project: Project) {
  try {
    await context?.build(project)
  } catch (e) {
    console.error("js:context:build:error", e)
    return
  }
  console.log("js:context:build", project)
}

export async function render() {
  await context?.render()
  console.log("js:context:render")
}

export async function reset() {
  if (!browser || !("gpu" in navigator)) return
  console.log("js:context:reset")
  context?.free()
  await init()
}

export async function stop() {
  if (!browser || !("gpu" in navigator)) return
  console.log("js:context:stop")
  context?.free()
}

export default context