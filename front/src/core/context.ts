import { Context as WasmContext } from '$gen/client/gpu_wasm_client'
import { wConfig, wFiles, wPrebuildDirty, wPrebuildResult } from '$stores'
import {
	get,
	writable,
	type Readable,
	type Subscriber,
	type Unsubscriber,
	type Writable
} from 'svelte/store'

export type ContextState = {
	ready: boolean
}

/**
 * Thin wrapper around gpu_wasm_client::Context
 */
class ClientContext implements Readable<ContextState> {
	private _store: Writable<ContextState>
	private _context?: WasmContext
	constructor() {
		this._store = writable({ ready: false })
	}
	subscribe(run: Subscriber<ContextState>, invalidate: any): Unsubscriber {
		return this._store.subscribe(run, invalidate)
	}
	async init(this: ClientContext) {
		// await init_client()
		try {
			this._context = await new WasmContext()
		} catch (e) {
			console.error('error in context:init: ', e)
			return
		}
		this._store.set({ ready: true })
	}
	async build() {
		if (!this._context) {
			console.error('Cannot build, wasm module isnt ready')
		}
		if (get(wPrebuildDirty)) this.prebuild()
		const prebuildResult = get(wPrebuildResult)
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
		if (!this._context) {
			console.error('Cannot prebuild, wasm module isnt ready')
		}
		const files = get(wFiles)
		let prebuildResult
		// try {
		//     prebuildResult = this._analyzer!.prebuild(files)
		// } catch (e) {
		//     console.error("js:context:prebuild:error", e)
		//     return
		// }

		wPrebuildDirty.set(false)
		// wPrebuildResult.set(prebuildResult)
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
		this._store.set({ ready: false })
	}
	resize(w: number, h: number) {
		this._context?.resize(w, h)
	}
}

const context: ClientContext = new ClientContext()
export default context
