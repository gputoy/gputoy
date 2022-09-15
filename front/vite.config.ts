import { sveltekit } from '@sveltejs/kit/vite';
import type { UserConfig } from 'vite';

const config: UserConfig = {
	plugins: [sveltekit()],
	server: {
		fs: {
			allow: [
				'./pkg/'
			]
		},
		host: true,
		port: 3000,
	},
};

export default config;
