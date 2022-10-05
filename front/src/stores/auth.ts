import vars from '$lib/vars';
import { writable } from "svelte/store";

interface User {
  id: string,
  username: string,
  email: string,
  fullname: string | undefined,
  bio: string | undefined,
  imageUrl: string | undefined,
  emailVerified: boolean,
  active: boolean,
}

export const user = writable<User | null>(null);

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
  });
  console.log("Returned res: ", loginRes);
  if (loginRes.status == 401) {
    user.set(null);
    console.log("Invalid credentials");
    return
  }
  if (loginRes.status != 200) {
    console.log("Unknown issue logging in: ", await loginRes.text());
    return
  }
  getSession()
}

export async function logout() {
  const logoutRes = await fetch(vars.API_PATH + 'logout', {
    method: 'POST',
    credentials: 'include'
  });
  console.log("Logout res: ", logoutRes);
  user.set(null);
}

export async function getSession() {
  const userRes = await fetch(vars.API_PATH + 'me', {
    method: 'GET',
    credentials: 'include'
  });

  if (userRes.status != 200) {
    console.log("Unknown issue fetching user: ", await userRes.text());
    return;
  }
  const json = await userRes.json();
  user.set(json as User);
}
