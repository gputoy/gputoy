import * as api from '$lib/core/api'
import { toast } from '@zerodevx/svelte-toast'
import Cookies from 'js-cookie'
import type { UpdateUserInfoArgs, UserInfoResponse } from 'src/generated/types'
import { get, writable } from "svelte/store"
import { dUserConfig, setUserConfig } from './userConfig'

export const wUser = writable<UserInfoResponse | null>(null)

export async function login(username_or_email: string, password: string) {
  const response = await api.login(username_or_email, password)
  if (response?.message) {
    // TODO: turn this awful alert into a presentable error message
    toast.push(`Recieved ${response.status} status on login response. Message: ${response.message}`)
    return
  }
  getSession()
}

export async function logout() {
  const response = await api.logout()
  if (response?.message)
    // TODO: turn this awful alert into a presentable error message
    toast.push(`Recieved ${response.status} status on logout response. Message: ${response.message}`)
  wUser.set(null)
}

export async function getSession() {
  let response
  if (Cookies.get('id'))
    response = await api.getSession()

  if (!response || 'message' in response) {
    let configString = localStorage?.getItem('config')
    let config = configString ? JSON.parse(configString) : undefined
    console.log('setting config', config)
    setUserConfig(config)
    return
  }
  wUser.set(response)
  setUserConfig(response.config ?? undefined)
}

export async function updateUser() {
  const user = get(wUser)
  const config = get(dUserConfig)
  const args: UpdateUserInfoArgs = {
    ...user,
    config,
  }
  const response = await api.updateUser(args)
  if ('message' in response) {
    // TODO: turn this awful alert into a presentable error message
    toast.push(`Recieved ${response.status} status on updateUser response. Message: ${response.message}`)
    return
  }
  wUser.set(response)
  setUserConfig(response.config ?? undefined)
}