import type { Handle } from '@sveltejs/kit'

export const handle: Handle = async ({ event, resolve }) => {
	const theme = event.cookies.get('theme')

	return await resolve(event, {
		transformPageChunk: ({ html }) => {
			// This defaults to dark theme in 'src/app.html'
			if (theme && theme !== 'dark')
				return html.replace('body class="dark"', `body class="${theme}"`)
			return html
		}
	})
}
