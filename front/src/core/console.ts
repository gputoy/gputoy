import type { ClientError, LogLevel as LogLevelString } from '$gen'
import { wConsole } from '$stores'
import * as wasm from '$wasm/common'
import { toast } from '@zerodevx/svelte-toast'
import type { Writable } from 'svelte/store'
import { pushAction } from './actions'

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
	out: (out: string) => void
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
	Echo = 1 << 5,
	/** Just print the content with no prefix. Cannot be filtered */
	Out = 1 << 6
}

export const LOG_PREFIX = new Map([
	[LogLevel.Trace, '[trace]'],
	[LogLevel.Debug, '[debug]'],
	[LogLevel.Info, '[info] '],
	[LogLevel.Warn, '[warn] '],
	[LogLevel.Error, '[error]'],
	[LogLevel.Echo, ' ~ '],
	[LogLevel.Out, '']
])

export const LOG_PREFIX_STYLES = new Map([
	[LogLevel.Trace, 'color:  var(--console-trace);'],
	[LogLevel.Debug, 'color:  var(--console-debug);'],
	[LogLevel.Info, 'color:  var(--console-info);'],
	[LogLevel.Warn, 'color:  var(--console-warn);'],
	[LogLevel.Error, 'color:  var(--console-error);'],
	[LogLevel.Echo, ''],
	[LogLevel.Out, '']
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
	function out(out: string) {
		console.update((console) => {
			console.push({
				level: LogLevel.Out,
				message: out
			})
			return console
		})
	}
	return { trace, debug, info, warn, error, echo, out }
}

export function submitCommand(command: string) {
	wConsole.echo(command)
	let result = wasm.action(command)
	if ('severity' in result) {
		handleClientError(result)
	} else {
		pushAction(result)
	}
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

export function prettyPrintJson(obj: any): string {
	const replacer = (
		_match: string,
		pIndent: any,
		pKey: any,
		pStrKey: any,
		pVal: any,
		pEnd: any
	) => {
		var key = '<span class=json-key>'
		var strKey = '<span class=json-key-str>'
		var val = '<span class=json-value>'
		var str = '<span class=json-string>'
		var r = pIndent || ''
		if (pKey) r = r + key + pKey.replace(/[": ]/g, '') + '</span>: '
		else if (pStrKey) r = r + strKey + pStrKey + '</span>: '
		if (pVal) r = r + (pVal[0] == '"' ? str : val) + pVal + '</span>'
		return r + (pEnd || '')
	}
	var jsonLine = /^( *)("[\w]+")?("[^"]+")?: ("[^"]*"|[\w.+-]*)?([,[{])?$/gm
	var value = JSON.stringify(obj, null, 4)
		.replace(/&/g, '&amp;')
		.replace(/\\"/g, '&quot;')
		.replace(/</g, '&lt;')
		.replace(/>/g, '&gt;')
		.replace(jsonLine, replacer)
	return '<pre>' + value + '</pre>'
}
