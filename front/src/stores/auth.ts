import { browser } from '$app/environment'
import vars from '$lib/vars'
import type { UpdateUserInfoArgs, UserInfoResponse } from 'src/generated/types'
import { get, writable } from "svelte/store"
import { dUserConfig, setUserConfig } from './userConfig'

export const wUser = writable<UserInfoResponse | null>(null)

export async function login(username_or_email: string, password: string) {
  const loginRes = await fetch(vars.API_PATH + 'login', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/x-www-form-urlencoded'
    },
    body: new URLSearchParams({
      username_or_email,
      password
    }),
    credentials: 'include'
  })
  console.log("Returned res: ", loginRes)
  if (loginRes.status == 401) {
    wUser.set(null)
    console.log("Invalid credentials")
    return
  }
  if (loginRes.status != 200) {
    console.log("Unknown issue logging in: ", await loginRes.text())
    return
  }
  getSession()
}

export async function logout() {
  const logoutRes = await fetch(vars.API_PATH + 'logout', {
    method: 'POST',
    credentials: 'include'
  })
  console.log("Logout res: ", logoutRes)
  wUser.set(null)
}

export async function getSession() {
  const userRes = await fetch(vars.API_PATH + 'me', {
    method: 'GET',
    credentials: 'include'
  })

  if (userRes.status != 200) {
    console.log("Unknown issue fetching user: ", await userRes.text())
    if (browser) {
      loadLocalConfig()
      return
    }
  }
  const user = await userRes.json()
  wUser.set(user)
  setUserConfig(user.config)
}

export function loadLocalConfig() {
  if (browser) {
    const localConfig = localStorage.getItem('config:local')
    if (localConfig)
      setUserConfig(JSON.parse(localConfig) ?? {})
    else
      setUserConfig({})
  } else {
    setUserConfig({})
  }
}

export async function updateUser() {
  const user = get(wUser)
  const config = get(dUserConfig)
  const body: UpdateUserInfoArgs = {
    ...user,
    config,
  }

  const updateUserResponse = await fetch(vars.API_PATH + 'me', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json'
    },
    body: JSON.stringify(body),
    credentials: 'include'
  })

  if (updateUserResponse.status != 200) {
    console.log("Unknown issue fetching user: ", await updateUserResponse.text())
    return
  }
  const updatedUser = await updateUserResponse.json()
  wUser.set(updatedUser)
}