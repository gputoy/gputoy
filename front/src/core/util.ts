export function delayedFocus(el: HTMLElement, wait: number) {
	setTimeout(() => {
		el.focus({ preventScroll: true })
	}, wait)
}
