/**
 *  Routing messages through the browser window.
 *  On load the frontend will assign implementations of
 *  these functions to the window which will update svelte stores.
 * 
 *  For more info view front/src/core/console.ts
 */

export function __trace_ext(log) {
    window.__trace_ext(log)
}
export function __debug_ext(log) {
    window.__debug_ext(log)
}
export function __info_ext(log) {
    window.__info_ext(log)
}
export function __warn_ext(log) {
    window.__warn_ext(log)
}
export function __error_ext(log) {
    window.__error_ext(log)
}