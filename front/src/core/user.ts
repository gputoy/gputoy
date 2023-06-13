import * as api from '$core/api'
// import { rPreferences, setPreferences } from '$core/preferences'
import { setPreferences } from '$core/preferences'
import type { UpdateUserInfoArgs, UserInfoResponse } from '$gen'
import { toast } from '@zerodevx/svelte-toast'
import Cookies from 'js-cookie'
import { get, type Writable } from 'svelte/store'

export type UserExtras = {
	login: (username_or_email: string, password: string) => void
	logout: () => void
	getSession: () => void
	updateUser: () => void
}
export function initUserMethods(
	user: Writable<UserInfoResponse | null>
): UserExtras {
	async function login(username_or_email: string, password: string) {
		const response = await api.login(username_or_email, password)
		if (response?.message) {
			// TODO: turn this awful alert into a presentable error message
			toast.push(
				`Recieved ${response.status} status on login response. Message: ${response.message}`
			)
			return
		}
		getSession()
	}

	async function logout() {
		const response = await api.logout()
		if (response?.message)
			// TODO: turn this awful alert into a presentable error message
			toast.push(
				`Recieved ${response.status} status on logout response. Message: ${response.message}`
			)
		user.set(null)
	}

	async function getSession() {
		let response
		if (Cookies.get('id')) response = await api.getSession()

		if (!response || 'message' in response) {
			const configString = localStorage?.getItem('config')
			const prefs = configString ? JSON.parse(configString) : undefined
			setPreferences(prefs)
			return
		}
		user.set(response)
		setPreferences(response.preferences)
	}

	async function updateUser() {
		const userInner = get(user)
		// const preferences = get(rPreferences)
		const args: UpdateUserInfoArgs = {
			...userInner
			// preferences,
		}
		const response = await api.updateUser(args)
		if ('message' in response) {
			// TODO: turn this awful alert into a presentable error message
			toast.push(
				`Recieved ${response.status} status on updateUser response. Message: ${response.message}`
			)
			return
		}
		user.set(response)
		setPreferences(response.preferences ?? undefined)
	}
	return { login, logout, getSession, updateUser }
}
