/**
 *  Routing messages through the browser window.
 *  On load the frontend will assign implementations of
 *  these functions to the window which will update svelte stores.
 * 
 *  For more info view front/src/core/client.ts
 */

export function __check_result_ext(result) {
    window.__check_result_ext(result)
}

export function __build_result_ext(result) {
    window.__build_result_ext(result)
}
