import { browser } from '$app/environment'
import { toast } from '@zerodevx/svelte-toast'
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
  console.log("js:context:build")
}

export async function render() {
  if (!context) {
    toast.push("No context to render!")
    return
  }
  context?.render()
  console.log("js:context:render")
}

export async function introspect(project: Project) {
  if (!context) {
    toast.push("Cannot introspect, context not ready")
    return
  }
  let compileResult = context.introspect(project)
  console.log("Compile result: ", compileResult)
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