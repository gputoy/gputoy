import vars from '$lib/vars'
import type { UserInfoResponse } from 'src/generated/types'
import { writable } from "svelte/store"

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
    return
  }
  const json = await userRes.json()
  wUser.set(json as UserInfoResponse)
}
