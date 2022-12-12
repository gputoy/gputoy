import { wConfig, wFiles, wPrebuildDirty, wPrebuildResult } from '$stores'
import { Context as WasmContext, default as init_client } from '$wasm/client/gpu_wasm_client'
import { Compiler as WasmCompiler, default as init_compiler } from '$wasm/compiler/gpu_wasm_compiler'
import { get, writable, type Readable, type Subscriber, type Unsubscriber, type Writable } from 'svelte/store'

export type ContextState = {
    ready: boolean
}

/**
 * Thin wrapper around gpu_wasm_client::Context
 */
class ClientContext implements Readable<ContextState> {
    private _store: Writable<ContextState>
    private _context?: WasmContext
    private _compiler?: WasmCompiler
    constructor() {
        this._store = writable({ ready: false })
    }
    subscribe(run: Subscriber<ContextState>, invalidate: any): Unsubscriber {
        return this._store.subscribe(run, invalidate)
    }
    async init(this: ClientContext) {
        await init_client()
        await init_compiler()
        try {

            this._context = await (new WasmContext())
            this._compiler = new WasmCompiler()
        } catch (e) {
            console.error("error in context:init: ", e)
            return
        }
        this._store.set({ ready: true })
    }
    async build() {
        if (!this._context || !this._compiler) {
            console.error("Cannot build, wasm module isnt ready")
        }
        if (get(wPrebuildDirty)) this.prebuild()
        const prebuildResult = get(wPrebuildResult)
        const runnerFile = wFiles.getFile(get(wConfig).runner ?? "")
        if (!runnerFile) {
            console.error("No runner file set, aborting.")
            return
        }
        const runner = JSON.parse(runnerFile.data)
        console.log(this._context)
        try {
            await this._context!.build(runner, prebuildResult)

        } catch (e) {
            console.error("js:context:build:error", e)
            return
        }
    }
    async prebuild() {
        if (!this._context || !this._compiler) {
            console.error("Cannot prebuild, wasm module isnt ready")
        }
        const files = get(wFiles)
        let prebuildResult
        try {
            prebuildResult = this._compiler!.prebuild(files)
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
        this._context = new WasmContext()
    }
    destroy() {
        this._context?.free()
        this._compiler?.free()
        this._store.set({ ready: false })
    }
    resize(w: number, h: number) {
        this._context?.resize(w, h)

    }
}

var context: ClientContext = new ClientContext()
export default context
