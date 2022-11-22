import { wFiles, wPrebuildDirty, wPrebuildResult } from '$stores'
import { Context as WasmContext, default as init_client } from '$wasm/client/gpu_wasm_client'
import { Compiler as WasmCompiler, default as init_compiler } from '$wasm/compiler/gpu_wasm_compiler'
import { get, writable, type Readable, type Subscriber, type Unsubscriber, type Writable } from 'svelte/store'

init_client().catch(e => console.error(e))
init_compiler().catch(e => console.error(e))

export type ContextState = {}

/**
 * Thin wrapper around gpu_wasm_client::Context
 */
class ClientContext implements Readable<ContextState> {
    private _store: Writable<ContextState>
    private _context: WasmContext
    private _compiler: WasmCompiler
    constructor() {
        this._context = new WasmContext()
        this._compiler = new WasmCompiler()
        this._store = writable({})
    }
    subscribe(run: Subscriber<ContextState>, invalidate: any): Unsubscriber {
        return this._store.subscribe(run, invalidate)
    }
    async build() {
        if (get(wPrebuildDirty)) this.prebuild()
        const prebuildResult = get(wPrebuildResult)
        try {
            await this._context.build(prebuildResult)
        } catch (e) {
            console.error("js:context:build:error", e)
            return
        }
    }
    async prebuild() {
        const files = get(wFiles)
        let prebuildResult
        try {
            prebuildResult = this._compiler.prebuild(files)
        } catch (e) {
            console.error("js:context:prebuild:error", e)
            return
        }
        wPrebuildDirty.set(false)
        wPrebuildResult.set(prebuildResult)
    }
    async render() {
        await this._context.render()
    }
    reset() {
        this._context.free()
        this._context = new WasmContext()
    }
}

export default ClientContext