/**
 * Its very important that monaco is NOT imported like this:
 *
 * `import * as monaco from 'monaco-editor'`
 *
 * Doing so will pollute the worker with tons of css that vite will handily
 * convert to hot-reload js modules that try to access the DOM.
 *
 * Feb 5th, 2023
 */

import GputoyWorker from '$monaco/wgsl/wgslWorker'
/** @ts-ignore */
import { initialize } from 'monaco-editor/esm/vs/editor/editor.worker'

/** @ts-ignore */
declare let self: DedicatedWorkerGlobalScope

self.onmessage = async (e: MessageEvent<string>) => {
	initialize((ctx: any, createData: any) => {
		return new GputoyWorker(ctx, createData)
	})
}

export default self
