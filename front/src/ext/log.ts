import { wConsole } from '$core/stores'

export function __trace_ext(log: string) {
	wConsole.trace(log)
}
export function __debug_ext(log: string) {
	wConsole.debug(log)
}
export function __info_ext(log: string) {
	wConsole.info(log)
}
export function __warn_ext(log: string) {
	wConsole.warn(log)
}
export function __error_ext(log: string) {
	wConsole.error(log)
}
