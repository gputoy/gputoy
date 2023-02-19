// import { nodeResolve } from '@rollup/plugin-node-resolve'
import { sveltekit } from '@sveltejs/kit/vite'
import * as path from 'path'
import type { UserConfig } from 'vite'
// import vpme from 'vite-plugin-monaco-editor'

const config: UserConfig = {
	build: {
		target: ['ES2020'],
		rollupOptions: {
			// plugins: [css({})],
			// external: ['monaco-editor'],
		}
	},
	optimizeDeps: {
		exclude: ['svelte-navigator', 'svelte-json-view', 'svelte-splitpanes', 'svelte-toast']
	},
	plugins: [sveltekit()],
	server: {
		fs: {
			allow: ['./pkg/']
		},
		port: 3000,
		host: true,
	},
	resolve: {
		alias: {
			$lib: path.resolve(__dirname, './src/lib'),
			$stores: path.resolve(__dirname, './src/core/stores'),
			$wasm: path.resolve(__dirname, './pkg'),
			$core: path.resolve(__dirname, './src/core'),
			$common: path.resolve(__dirname, './src/core/common'),
			$monaco: path.resolve(__dirname, './src/monaco')
		}
	},
	// worker: {
	// 	format: 'es'
	// }
}

export default config
