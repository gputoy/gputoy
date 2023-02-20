// Contains fetch calls for all api endpoints in gpu-back
// All endpoints are contained in one file to make syncing with
// gpu-back simpler.

import type * as types from '$common'
import { API_URL } from '$core/consts'

export type ResponseError = {
	message: string
	status: number
}

export type Response<T> = Promise<T | ResponseError>

// -------------------------------------------------
//                  User api calls
// -------------------------------------------------

/**
 * POST /signup
 * @param args
 * @returns NewUserResponse | error
 */
export async function signUp(
	args: types.NewUser
): Response<types.NewUserResponse> {
	const response = await fetch(API_URL + 'signup', {
		method: 'POST',
		headers: {
			'Content-Type': 'application/x-www-form-urlencoded'
		},
		body: new URLSearchParams(args as {}),
		mode: 'cors',
	})
	const json = await response.json()
	if (response.status != 200) return json.message
	return json
}

/**
 * POST /login
 * @param usernameOrEmail
 * @param password
 * @returns undefined | error
 */
export async function login(
	usernameOrEmail: string,
	password: string
): Response<undefined> {
	const loginRes = await fetch(API_URL + 'login', {
		method: 'POST',
		headers: {
			'Content-Type': 'application/x-www-form-urlencoded'
		},
		body: new URLSearchParams({
			usernameOrEmail,
			password
		}),
		credentials: 'include',
		mode: 'cors',
	})
	if (loginRes.status != 200)
		return {
			message: (await loginRes.json()).message,
			status: loginRes.status
		}
}

/**
 * POST /logout
 * @returns undefined | error
 */
export async function logout(): Response<undefined> {
	const logoutRes = await fetch(API_URL + 'logout', {
		method: 'POST',
		credentials: 'include',
		mode: 'cors',
	})
	if (logoutRes.status != 200)
		return {
			message: (await logoutRes.json()).message,
			status: logoutRes.status
		}
}

/**
 * GET /me
 * @returns UserInfoResponse | error
 */
export async function getSession(): Response<types.UserInfoResponse> {
	const userRes = await fetch(API_URL + 'me', {
		method: 'GET',
		credentials: 'include'
	})
	if (userRes.status != 200) {
		return {
			message: 'Unauthorized',
			status: userRes.status
		}
	}
	const user = await userRes.json()
	return user
}

/**
 * POST /me
 * @param args
 * @returns UserInfoResponse | error
 */
export async function updateUser(
	args: types.UpdateUserInfoArgs
): Response<types.UserInfoResponse> {
	const updateUserRes = await fetch(API_URL + 'me', {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json'
		},
		body: JSON.stringify(args),
		credentials: 'include'
	})

	const updatedUser = await updateUserRes.json()
	if (updatedUser.status != 200) {
		return {
			message: updatedUser.message,
			status: updateUserRes.status
		}
	}
	return updatedUser
}

// -------------------------------------------------
//                  Project api calls
// -------------------------------------------------

/**
 * GET /project/{projectid}
 * @param projectid
 * @returns ProjectResponse | error
 */
export async function getProject(
	projectid: string
): Response<types.ProjectResponse> {
	const projectRes = await fetch(API_URL + 'project/' + projectid, {
		method: 'GET',
		credentials: 'include'
	})

	const project = await projectRes.json()
	if (projectRes.status != 200) {
		return {
			message: project.message,
			status: projectRes.status
		}
	}
	return project
}

/**
 * GET /project/user/{userid}
 * @param userid
 * @returns ProjectResponse[] | error
 */
export async function getUserProjects(
	userid: string
): Response<types.ProjectResponse[]> {
	const userProjectsRes = await fetch(API_URL + 'project/user/' + userid, {
		method: 'GET',
		credentials: 'include'
	})
	const userProjects = await userProjectsRes.json()

	if (userProjectsRes.status != 200) {
		return {
			message: userProjects.message,
			status: userProjectsRes.status
		}
	}
	return userProjects
}

/**
 * POST /project
 * @param args
 * @returns ProjectResponse | error
 */
export async function updateProject(
	args: types.ProjectUpsert
): Response<types.ProjectResponse> {
	const projectRes = await fetch(API_URL + 'project', {
		method: 'POST',
		headers: { 'Content-Type': 'application/json' },
		credentials: 'include',
		body: JSON.stringify(args)
	})

	const project = await projectRes.json()
	if (projectRes.status != 200) {
		return {
			message: project.message,
			status: projectRes.status
		}
	}
	return project
}
