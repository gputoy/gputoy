import { editor, Uri, type IDisposable } from 'monaco-editor'
import type { LanguageServiceDefaults } from './index'
import type WgslWorker from './wgslWorker'

const STOP_WHEN_IDLE_FOR = 2 * 60 * 1000 // 2min

export default class WorkerManager {
    private _defaults: LanguageServiceDefaults
    private _lastUsedTime: number
    private _configChangeListener: IDisposable

    private _worker: editor.MonacoWebWorker<WgslWorker> | null
    private _client: Promise<WgslWorker> | null

    constructor(modeId: string, defaults: LanguageServiceDefaults) {
        this._defaults = defaults
        this._worker = null
        this._client = null
        this._lastUsedTime = 0
        this._configChangeListener = this._defaults.onDidChange(() => this._stopWorker())
    }

    private _stopWorker(): void {
        if (this._worker) {
            this._worker.dispose()
            this._worker = null
        }
        this._client = null
    }

    dispose(): void {
        this._configChangeListener.dispose()
        this._stopWorker()
    }

    private _checkIfIdle(): void {
        if (!this._worker) {
            return
        }
        let timePassedSinceLastUsed = Date.now() - this._lastUsedTime
        if (timePassedSinceLastUsed > STOP_WHEN_IDLE_FOR) {
            this._stopWorker()
        }
    }

    private _getClient(): Promise<WgslWorker> {
        this._lastUsedTime = Date.now()

        if (!this._client) {
            this._worker = editor.createWebWorker<WgslWorker>({
                moduleId: 'WgslWorker',
                label: this._defaults.languageId,
                createData: {
                    languageSettings: this._defaults.diagnosticsOptions,
                    languageId: this._defaults.languageId,
                }
            })

            this._client = <Promise<WgslWorker>>(<any>this._worker.getProxy())
        }

        return this._client
    }

    getLanguageServiceWorker(...resources: Uri[]): Promise<WgslWorker> {
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