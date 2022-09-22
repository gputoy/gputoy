import { sveltekit } from '@sveltejs/kit/vite';
import path from 'path';
import type { UserConfig } from 'vite';

const config: UserConfig = {
	optimizeDeps: {
		exclude: ['svelte-navigator']
	},
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
	resolve: {
		alias: {
			"$lib": path.resolve(__dirname, './src/lib'),
			"$stores": path.resolve(__dirname, './src/stores'),
		}
	}
};

export default config;
