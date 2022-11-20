import type { UpdateUserInfoArgs, UserInfoResponse } from '$common'
import * as api from '$core/api'
import { setUserPrefs } from '$core/preferences'
import { dUserPrefs } from '$stores'
import { toast } from '@zerodevx/svelte-toast'
import Cookies from 'js-cookie'
import { get, type Writable } from "svelte/store"

export type UserExtras = {
    login: (username_or_email: string, password: string) => void
    logout: () => void
    getSession: () => void
    updateUser: () => void
}
export function initUserMethods(user: Writable<UserInfoResponse | null>): UserExtras {

    async function login(username_or_email: string, password: string) {
        const response = await api.login(username_or_email, password)
        if (response?.message) {
            // TODO: turn this awful alert into a presentable error message
            toast.push(`Recieved ${response.status} status on login response. Message: ${response.message}`)
            return
        }
        getSession()
    }

    async function logout() {
        const response = await api.logout()
        if (response?.message)
            // TODO: turn this awful alert into a presentable error message
            toast.push(`Recieved ${response.status} status on logout response. Message: ${response.message}`)
        user.set(null)
    }

    async function getSession() {
        let response
        if (Cookies.get('id'))
            response = await api.getSession()

        if (!response || 'message' in response) {
            let configString = localStorage?.getItem('config')
            let config = configString ? JSON.parse(configString) : undefined
            setUserPrefs(config)
            return
        }
        user.set(response)
        setUserPrefs(response.config ?? undefined)
    }

    async function updateUser() {
        const userInner = get(user)
        const config = get(dUserPrefs)
        const args: UpdateUserInfoArgs = {
            ...userInner,
            config,
        }
        const response = await api.updateUser(args)
        if ('message' in response) {
            // TODO: turn this awful alert into a presentable error message
            toast.push(`Recieved ${response.status} status on updateUser response. Message: ${response.message}`)
            return
        }
        user.set(response)
        setUserPrefs(response.config ?? undefined)
    }
    return { login, logout, getSession, updateUser }
}

