import type { PrebuildResult } from '$gen'
import * as wasm from '$gen/client/gpu_wasm_client'
import { wWorkerInternal } from '$monaco/wgsl/wgslMode'
import { wConfig, wFiles, wPrebuildDirty, wPrebuildResult } from '$stores'
import {
  get,
  writable,
  type Readable,
  type Subscriber,
  type Unsubscriber,
  type Writable
} from 'svelte/store'
import { WASM_CLIENT_URL } from './consts'

export type ContextState = {
  ready: boolean
}

/**
 * Thin wrapper around gpu_wasm_client::Context
 */
class ClientContext implements Readable<ContextState> {
  private _store: Writable<ContextState>
  private _context?: wasm.Context
  constructor() {
    this._store = writable({ ready: false })
  }
  subscribe(run: Subscriber<ContextState>, invalidate: any): Unsubscriber {
    return this._store.subscribe(run, invalidate)
  }
  async init(this: ClientContext) {
    await wasm.default(WASM_CLIENT_URL)
    try {
      this._context = await new wasm.Context()
    } catch (e) {
      console.error('error in context:init: ', e)
      return
    }
    this._store.set({ ready: true })
  }
  async build(prebuildResult: PrebuildResult) {
    console.log('build')
    if (!this._context) {
      console.error('Cannot build, wasm module isnt ready')
    }
    if (get(wPrebuildDirty)) this.prebuild()
    const runnerFile = wFiles.getFile(get(wConfig).runner ?? '')
    if (!runnerFile) {
      console.error('No runner file set, aborting.')
      return
    }
    const runner = JSON.parse(runnerFile.data)
    try {
      await this._context!.build(runner, prebuildResult)
    } catch (e) {
      console.error('js:context:build:error', e)
      return
    }
  }
  async prebuild() {
    console.log('prebuild')
    if (!this._context) {
      console.error('Cannot prebuild, wasm module isnt ready')
    }
    let prebuildResult
    try {
      prebuildResult = await wWorkerInternal.tryBuild()
    } catch (e) {
      console.error("js:context:prebuild:error", e)
      return
    }

    wPrebuildDirty.set(false)
    wPrebuildResult.set(prebuildResult)
  }
  async render() {
    await this._context!.render()
  }
  reset() {
    this._context?.free()
    this._context = new wasm.Context()
  }
  destroy() {
    this._context?.free()
    this._store.set({ ready: false })
  }
  resize(w: number, h: number) {
    this._context?.resize(w, h)
  }
}

const context: ClientContext = new ClientContext()
export default context

