import { wConsole, wConsoleHistory } from '$stores'
import { toast } from '@zerodevx/svelte-toast'
import type { Writable } from 'svelte/store'
import actionSchema from '../../../schemas/Action.json'
import { pushAction } from './actions'

const CONSOLE_ACTION_NAMES = actionSchema.oneOf.map((actionDef) => actionDef.properties.ty.enum[0])
const CONSOLE_COMPLETIONS = actionSchema.oneOf.map((actionDef) => ({
    action: actionDef.properties.ty.enum[0],
    description: actionDef.description,
    args: actionDef.properties.c
})) as ReadonlyArray<ConsoleCompletion>

export type Log = {
    level: LogLevel,
    message: string,
}
export type ConsoleCompletion = {
    action: string,
    description: string,
    args: any,
}

export type ConsoleExtras = {
    trace: (trace: string) => void
    debug: (debug: string) => void
    info: (info: string) => void
    warn: (warn: string) => void
    error: (error: string) => void
    echo: (echo: string) => void
}

export enum LogLevel {
    /** Development use only */
    Trace = 1 << 0,
    /** User won't need to know unless they need to */
    Debug = 1 << 1,
    /** Something the user ought to know */
    Info = 1 << 2,
    /** Should be corrected, but the execution can continue */
    Warn = 1 << 3,
    /** Execution had to halt */
    Error = 1 << 4,
    /** Echo prompt after use submits command. Cannot be filtered. */
    Echo = 1 << 5
}

export const LOG_PREFIX = new Map([
    [LogLevel.Trace, '[trace] '],
    [LogLevel.Debug, '[debug] '],
    [LogLevel.Info, '[info]  '],
    [LogLevel.Warn, '[warn]  '],
    [LogLevel.Error, '[error] '],
    [LogLevel.Echo, 'hello'],
])

export const LOG_PREFIX_STYLES = new Map([
    [LogLevel.Trace, 'color:  var(--console-trace);'],
    [LogLevel.Debug, 'color:  var(--console-debug);'],
    [LogLevel.Info, 'color:  var(--console-info);'],
    [LogLevel.Warn, 'color:  var(--console-warn);'],
    [LogLevel.Error, 'color:  var(--console-error);'],
    [LogLevel.Echo, 'display: none;'],
])



/**
 * Generate console completions list based on what text is currently in console.
 * @param consoleCommand text in console
 * @returns 
 */
export function generateCompletions(consoleCommand: string): ConsoleCompletion[] {
    // get command string and args as string
    let [command, ...args] = consoleCommand.trim().split(/\s+/)
    if (!command) return []
    // if command is fully typed, just return that as a completion 
    let foundIndex = CONSOLE_ACTION_NAMES.indexOf(command)
    if (foundIndex >= 0) {
        return [CONSOLE_COMPLETIONS[foundIndex]]
    }

    return CONSOLE_COMPLETIONS
        .filter(completion => completion.action.includes(command))
}

/**
 * Essentially a wrapper around pushAction, but also handles book-keeping
 * for the console store, along with argument extraction.
 * @param consoleCommand command string from console
 * @returns void
 */
export function submitConsoleComand(consoleCommand: string) {
    let echo = " ~ " + consoleCommand
    // push this command to console history so user may use
    // arrow keys to quickly redo previous commands
    wConsoleHistory.update(history => {
        let lastCommand = history.pop()
        if (!lastCommand || lastCommand == consoleCommand) history.push(consoleCommand)
        else history.push(lastCommand, consoleCommand)
        return history
    })
    // log the users input to the console
    wConsole.echo(echo)
    // get command string and args as string
    let [command, ...args] = consoleCommand.trim().split(/\s+/)
    if (!command) return
    // if command is fully typed, just return that as a completion 

    const actionIndex = CONSOLE_ACTION_NAMES.indexOf(command)
    if (actionIndex < 0) {
        wConsole.error(command + " not found.")
        return
    }

    const argsSchema = CONSOLE_COMPLETIONS[actionIndex].args
    if (argsSchema) {
        if ("type" in argsSchema && argsSchema.type == 'string') {
            let arg = args[0]
            if (!arg) {
                wConsole.error(command + " requires one string argument")
                return
            }
            /** @ts-ignore */
            pushAction({ ty: command, c: arg })

        }
        else if ("$schema" in argsSchema) {
            toast.push("Commands with more than one argument not supported yet.")
        }

    } else {
        // No arguments to action
        /** @ts-ignore */
        pushAction({ ty: command })
    }



}

export function initConsoleMethods(console: Writable<Log[]>): ConsoleExtras {
    function trace(trace: string) {
        console.update(console => {
            console.push({
                level: LogLevel.Trace,
                message: trace,
            })
            return console
        })
    }
    function debug(debug: string) {
        console.update(console => {
            console.push({
                level: LogLevel.Debug,
                message: debug,
            })
            return console
        })
    }
    function info(info: string) {
        console.update(console => {
            console.push({
                level: LogLevel.Info,
                message: info,
            })
            return console
        })
    }
    function warn(warn: string) {
        console.update(console => {
            console.push({
                level: LogLevel.Warn,
                message: warn,
            })
            return console
        })
    }
    function error(error: string) {
        console.update(console => {
            console.push({
                level: LogLevel.Error,
                message: error,
            })
            return console
        })
    }
    function echo(echo: string) {

        console.update(console => {
            console.push({
                level: LogLevel.Echo,
                message: echo,
            })
            return console
        })
    }
    return { trace, debug, info, warn, error, echo }
}