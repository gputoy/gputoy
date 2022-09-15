// @ts-nocheck
import { error } from '@sveltejs/kit';
import { api } from './api';
import type { PageServerLoad, Actions } from './$types';

type Todo = {
	uid: string;
	created_at: Date;
	text: string;
	done: boolean;
	pending_delete: boolean;
};

export const load = async ({ locals }: Parameters<PageServerLoad>[0]) => {
	// locals.userid comes from src/hooks.js
	const response = await api('GET', `todos/${locals.userid}`);

	if (response.status === 404) {
		// user hasn't created a todo list.
		// start with an empty array
		return {
			todos: [] as Todo[]
		};
	}

	if (response.status === 200) {
		return {
			todos: (await response.json()) as Todo[]
		};
	}

	throw error(response.status);
};

export const actions = {
	add: async ({ request, locals }: import('./$types').RequestEvent) => {
		const form = await request.formData();

		await api('POST', `todos/${locals.userid}`, {
			text: form.get('text')
		});
	},
	edit: async ({ request, locals }: import('./$types').RequestEvent) => {
		const form = await request.formData();

		await api('PATCH', `todos/${locals.userid}/${form.get('uid')}`, {
			text: form.get('text')
		});
	},
	toggle: async ({ request, locals }: import('./$types').RequestEvent) => {
		const form = await request.formData();

		await api('PATCH', `todos/${locals.userid}/${form.get('uid')}`, {
			done: !!form.get('done')
		});
	},
	delete: async ({ request, locals }: import('./$types').RequestEvent) => {
		const form = await request.formData();

		await api('DELETE', `todos/${locals.userid}/${form.get('uid')}`);
	}
};
;null as any as Actions;