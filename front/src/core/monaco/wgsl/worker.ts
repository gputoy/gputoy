import GputoyWorker from '$core/monaco/wgsl/wgslWorker'
/** @ts-ignore */
import { initialize } from 'monaco-editor/esm/vs/editor/editor.worker'

declare var self: DedicatedWorkerGlobalScope

self.onmessage = (e: MessageEvent<string>) => {
    initialize((ctx: any, createData: any) => {
        return new GputoyWorker(ctx, createData)
    })
}

export default self