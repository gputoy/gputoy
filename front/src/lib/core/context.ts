import { browser } from '$app/environment'
import { toast } from '@zerodevx/svelte-toast'
import type { Files, Project } from 'src/generated/types'

import { Context, default as init_client } from '$wasm/client/gpu_wasm_client'
import { Compiler, default as init_compiler } from '$wasm/compiler/gpu_wasm_compiler'

var context: Context | undefined = undefined
var compiler: Compiler | undefined = undefined

export async function init() {
  if (!browser || !("gpu" in navigator)) return
  await init_client()
  await init_compiler()

  context = await new Context()
  compiler = new Compiler()
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

export async function introspect(files: Files) {
  if (!context) {
    toast.push("Cannot introspect, context not ready")
    return
  }
  let compileResult = compiler?.analyze(files)
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