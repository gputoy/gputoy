import type { ClientError, LogLevel as LogLevelString } from '$gen'
import rootSchema from '$gen/root-schema.json'
import { wConsole } from '$stores'
import { toast } from '@zerodevx/svelte-toast'
import type { Writable } from 'svelte/store'

// Add private logging methods to window so wasm modules can use it
// without having to import anything from /front.
//
// Methods only meant to be used within gpu-log/log_ext.js
if (typeof window !== 'undefined') {
	Object.assign(window, {
		__trace_ext: function (log: string) {
			wConsole.trace(log)
		},
		__debug_ext: function (log: string) {
			wConsole.debug(log)
		},
		__info_ext: function (log: string) {
			wConsole.info(log)
		},
		__warn_ext: function (log: string) {
			wConsole.warn(log)
		},
		__error_ext: function (log: string) {
			wConsole.error(log)
		}
	})
}

const actionSchema = rootSchema.definitions.Action
export type Log = {
	level: LogLevel
	message: string
}

export type ConsoleExtras = {
	trace: (trace: string) => void
	debug: (debug: string) => void
	info: (info: string) => void
	warn: (warn: string) => void
	error: (error: string) => void
	echo: (echo: string) => void
}

export function toLogLevel(level: LogLevelString) {
	switch (level) {
		case 'trace':
			return LogLevel.Trace
		case 'debug':
			return LogLevel.Debug
		case 'info':
			return LogLevel.Info
		case 'warn':
			return LogLevel.Warn
		case 'error':
			return LogLevel.Error
		default:
			return 0
	}
}

export enum LogLevel {
	Trace = 1 << 0,
	Debug = 1 << 1,
	Info = 1 << 2,
	Warn = 1 << 3,
	Error = 1 << 4,
	/** Echo prompt after use submits command. Cannot be filtered. */
	Echo = 1 << 5
}

export const LOG_PREFIX = new Map([
	[LogLevel.Trace, '[trace]'],
	[LogLevel.Debug, '[debug]'],
	[LogLevel.Info, '[info] '],
	[LogLevel.Warn, '[warn] '],
	[LogLevel.Error, '[error]'],
	[LogLevel.Echo, ' ~ ']
])

export const LOG_PREFIX_STYLES = new Map([
	[LogLevel.Trace, 'color:  var(--console-trace);'],
	[LogLevel.Debug, 'color:  var(--console-debug);'],
	[LogLevel.Info, 'color:  var(--console-info);'],
	[LogLevel.Warn, 'color:  var(--console-warn);'],
	[LogLevel.Error, 'color:  var(--console-error);'],
	[LogLevel.Echo, '']
])

export function initConsoleMethods(console: Writable<Log[]>): ConsoleExtras {
	function trace(trace: string) {
		console.update((console) => {
			console.push({
				level: LogLevel.Trace,
				message: trace
			})
			return console
		})
	}
	function debug(debug: string) {
		console.update((console) => {
			console.push({
				level: LogLevel.Debug,
				message: debug
			})
			return console
		})
	}
	function info(info: string) {
		console.update((console) => {
			console.push({
				level: LogLevel.Info,
				message: info
			})
			return console
		})
	}
	function warn(warn: string) {
		console.update((console) => {
			console.push({
				level: LogLevel.Warn,
				message: warn
			})
			return console
		})
	}
	function error(error: string) {
		console.update((console) => {
			console.push({
				level: LogLevel.Error,
				message: error
			})
			return console
		})
	}
	function echo(echo: string) {
		console.update((console) => {
			console.push({
				level: LogLevel.Echo,
				message: echo
			})
			return console
		})
	}
	return { trace, debug, info, warn, error, echo }
}

export function handleClientError(clientError: ClientError) {
	let fullMessage = `[${clientError.source}] ${clientError.message}`
	if (clientError.destination == 'console') {
		if (clientError.severity == 'error') {
			wConsole.error(fullMessage)
		} else {
			wConsole.warn(fullMessage)
		}
	} else {
		toast.push(fullMessage, {
			dismissable: true,
			duration: 0,
			intro: {
				x: -100
			}
		})
	}
}
