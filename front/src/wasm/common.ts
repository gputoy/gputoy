import { WASM_COMMON_URL } from '$core/consts'
import type * as common from '$gen'
import * as wasm from '$gen/common/gpu_wasm_common'

export type Result<T> = T | common.ClientError
export function action(cmd: string): Result<common.Action> {
	try {
		return wasm.__action(cmd)
	} catch (e) {
		return e as common.ClientError
	}
}
export function completion(
	cmd: string,
	cursorCharPos: number
): Result<common.CompletionInfo> {
	try {
		return wasm.__action_completion(cmd, cursorCharPos)
	} catch (e) {
		return e as common.ClientError
	}
}
export function path(path: string): Result<common.Path> {
	try {
		return wasm.__path(path)
	} catch (e) {
		return e as common.ClientError
	}
}
export function filePath(path: string): Result<common.FilePath> {
	try {
		return wasm.__file_path(path)
	} catch (e) {
		return e as common.ClientError
	}
}

async function init() {
	await wasm.default(WASM_COMMON_URL)
}

init()
