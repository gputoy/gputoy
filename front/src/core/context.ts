import { browser } from '$app/environment'
import type { Files, PrebuildResult, Project } from '$common'
import { toast } from '@zerodevx/svelte-toast'

import { wPrebuildDirty, wPrebuildResult } from '$stores'
import { Context, default as init_client } from '$wasm/client/gpu_wasm_client'
import { Compiler, default as init_compiler } from '$wasm/compiler/gpu_wasm_compiler.js'
import { get } from 'svelte/store'

var context: Context | undefined = undefined
var compiler: Compiler | undefined = undefined

export function getContextHealth() {
  return context !== undefined
}

export function getCompilerHealth() {
  return compiler !== undefined
}

export async function init() {
  if (!browser || !("gpu" in navigator)) return
  await init_client()
  await init_compiler()

  context = await new Context()
  compiler = new Compiler()
  console.log("js:context:init", context)
}

export async function build(project: Project) {
  if (get(wPrebuildDirty)) prebuild(project.files)
  const prebuildResult = get(wPrebuildResult)
  try {
    await context?.build(project, prebuildResult)
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

export async function prebuild(files: Files) {
  if (!compiler) {
    toast.push("Cannot introspect, compiler not ready")
    return
  }
  let prebuildResult: PrebuildResult = compiler.prebuild(files)
  console.log('prebuild res', prebuildResult)
  wPrebuildResult.set(prebuildResult)
  wPrebuildDirty.set(false)
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