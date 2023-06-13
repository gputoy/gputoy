import type * as monaco from 'monaco-editor'
import { editor } from 'monaco-editor'
import type { LanguageServiceDefaults } from './index'
import type WgslWorker from './wgslWorker'

import { WASM_ANALYZER_URL } from '$core/consts'
import { wFiles } from '$stores'
import { get } from 'svelte/store'
import type { CreateData } from './wgslWorker'

const STOP_WHEN_IDLE_FOR = 2 * 60 * 1000 // 2min

export default class WorkerManager {
	private _defaults: LanguageServiceDefaults
	private _lastUsedTime: number
	private _configChangeListener: monaco.IDisposable
	private _idleCheckInterval: number

	private _worker: monaco.editor.MonacoWebWorker<WgslWorker> | null
	private _client: Promise<WgslWorker> | null

	constructor(modeId: string, defaults: LanguageServiceDefaults) {
		this._defaults = defaults
		this._worker = null
		this._client = null
		/** @ts-ignore */
		this._idleCheckInterval = window?.setInterval(
			() => this._checkIfIdle(),
			30 * 1000
		)
		this._lastUsedTime = 0
		this._configChangeListener = this._defaults.onDidChange(() =>
			this._stopWorker()
		)
	}

	private _stopWorker(): void {
		if (this._worker) {
			this._worker.dispose()
			this._worker = null
		}
		this._client = null
	}

	dispose(): void {
		clearInterval(this._idleCheckInterval)
		this._configChangeListener.dispose()
		this._stopWorker()
	}

	private _checkIfIdle(): void {
		if (!this._worker) {
			return
		}
		const timePassedSinceLastUsed = Date.now() - this._lastUsedTime
		if (timePassedSinceLastUsed > STOP_WHEN_IDLE_FOR) {
			this._stopWorker()
		}
	}

	private _getClient(): Promise<WgslWorker> {
		this._lastUsedTime = Date.now()

		if (!this._client) {
			this._worker = editor.createWebWorker<WgslWorker>({
				moduleId: 'wgslWorker',
				label: this._defaults.languageId,
				createData: <CreateData>{
					languageSettings: this._defaults.diagnosticsOptions,
					languageId: this._defaults.languageId,
					wasmModuleUrl: WASM_ANALYZER_URL,
					initFiles: get(wFiles)
				},
				host: {
					postBackHere: (arg: string) => {}
				}
			})

			this._client = <Promise<WgslWorker>>(<any>this._worker.getProxy())
		}

		return this._client
	}

	getLanguageServiceWorker(...resources: monaco.Uri[]): Promise<WgslWorker> {
		let _client: WgslWorker
		return this._getClient()
			.then((client) => {
				_client = client
			})
			.then((_) => {
				if (this._worker) {
					return this._worker.withSyncedResources(resources)
				}
			})
			.then((_) => _client)
	}
}
