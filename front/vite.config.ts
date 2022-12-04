import { sveltekit } from '@sveltejs/kit/vite'
import path from 'path'
import type { UserConfig } from 'vite'
const config: UserConfig = {
	build: {
		target: ['es2020'],
	},
	optimizeDeps: {
		exclude: ['svelte-navigator', 'svelte-json-view'],
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
			"$stores": path.resolve(__dirname, './src/core/stores'),
			"$wasm": path.resolve(__dirname, "./pkg"),
			"$core": path.resolve(__dirname, "./src/core"),
			"$common": path.resolve(__dirname, "./src/core/common")
		}
	},
}

export default config
