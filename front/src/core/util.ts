export function delayedFocus(el: HTMLElement, wait: number) {
	setTimeout(() => {
		el.focus({ preventScroll: true })
	}, wait)
}

export function searchSplit(
	value: string,
	search: string
): [string, string, string] | null {
	let start = value.indexOf(search)
	if (start < 0) return null
	let end = start + search.length
	return [
		value.substring(0, start),
		value.substring(start, end),
		value.substring(end)
	] as [string, string, string]
}
