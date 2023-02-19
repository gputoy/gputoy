import { API_PATH } from '$core/consts'
import { error, type RequestHandler } from '@sveltejs/kit'

export const load: RequestHandler<{ pid: string }> = async function load({
	cookies,
	params
}) {
	if (params.pid == undefined) return error(404, 'Project not found')
	const projectResponse = await fetch(API_PATH + 'project/' + params.pid, {
		method: 'GET',
		credentials: 'include',
		headers: {
			cookie: `id=${cookies.get('id')}`
		}
	})
	const project = await projectResponse.json()
	if (projectResponse.status == 401) {
		throw error(401, 'Project is private')
	}
	if (projectResponse.status == 404) {
		throw error(404, 'Project not found')
	}
	if (projectResponse.status != 200) {
		throw error(
			500,
			'Something went wrong fetching project: ' + project.message
		)
	}

	return project
}
