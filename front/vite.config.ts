// import { nodeResolve } from '@rollup/plugin-node-resolve'
import { sveltekit } from '@sveltejs/kit/vite'
import * as path from 'path'
import Icons from 'unplugin-icons/vite'
import type { UserConfig } from 'vite'

const config: UserConfig = {
	build: {
		target: ['ES2020']
	},
	optimizeDeps: {
		exclude: [
			'svelte-navigator',
			'svelte-json-view',
			'svelte-splitpanes',
			'svelte-toast'
		]
	},
	plugins: [
		sveltekit(),
		Icons({
			compiler: 'svelte'
		})
	],
	server: {
		fs: {
			allow: ['./pkg/', './generated/']
		},
		port: 3000,
		host: true
	},
	resolve: {
		alias: {
			$lib: path.resolve(__dirname, './src/lib'),
			$stores: path.resolve(__dirname, './src/core/stores'),
			$core: path.resolve(__dirname, './src/core'),
			$common: path.resolve(__dirname, './generated/common'),
			$monaco: path.resolve(__dirname, './src/monaco'),
			$gen: path.resolve(__dirname, './generated'),
			$wasm: path.resolve(__dirname, './src/wasm')
		}
	}
}

export default config
