import { e as error } from "../../../chunks/index2.js";
const base = "https://api.svelte.dev";
function api(method, resource, data) {
  return fetch(`${base}/${resource}`, {
    method,
    headers: {
      "content-type": "application/json"
    },
    body: data && JSON.stringify(data)
  });
}
const load = async ({ locals }) => {
  const response = await api("GET", `todos/${locals.userid}`);
  if (response.status === 404) {
    return {
      todos: []
    };
  }
  if (response.status === 200) {
    return {
      todos: await response.json()
    };
  }
  throw error(response.status);
};
const actions = {
  add: async ({ request, locals }) => {
    const form = await request.formData();
    await api("POST", `todos/${locals.userid}`, {
      text: form.get("text")
    });
  },
  edit: async ({ request, locals }) => {
    const form = await request.formData();
    await api("PATCH", `todos/${locals.userid}/${form.get("uid")}`, {
      text: form.get("text")
    });
  },
  toggle: async ({ request, locals }) => {
    const form = await request.formData();
    await api("PATCH", `todos/${locals.userid}/${form.get("uid")}`, {
      done: !!form.get("done")
    });
  },
  delete: async ({ request, locals }) => {
    const form = await request.formData();
    await api("DELETE", `todos/${locals.userid}/${form.get("uid")}`);
  }
};
export {
  actions,
  load
};
