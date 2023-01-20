import type { Writable } from "svelte/store"

export type RunState = {
    playing: boolean,
}

export type RunStateExtras = {
    play: () => void,
    pause: () => void,
    playPause: () => void,
}
export function initRunStateMethods(runstate: Writable<RunState>): RunStateExtras {
    function play() {
        runstate.update(runstate => {
            runstate.playing = true
            return runstate
        })
    }

    function pause() {
        runstate.update(runstate => {
            runstate.playing = false
            return runstate
        })
    }

    function playPause() {
        runstate.update(runstate => {
            runstate.playing = !runstate.playing
            return runstate
        })
    }

    return { play, pause, playPause }
}