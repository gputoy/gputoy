export const manifest = {
	appDir: "_app",
	assets: new Set(["favicon.png","robots.txt","svelte-welcome.png","svelte-welcome.webp"]),
	mimeTypes: {".png":"image/png",".txt":"text/plain",".webp":"image/webp"},
	_: {
		entry: {"file":"_app/immutable/start-b9fde2ad.js","imports":["_app/immutable/start-b9fde2ad.js","_app/immutable/chunks/index-66220184.js","_app/immutable/chunks/singletons-b3dc1656.js","_app/immutable/chunks/index-e366d1ab.js"],"stylesheets":[]},
		nodes: [
			() => import('.//nodes/0.js'),
			() => import('.//nodes/1.js'),
			() => import('.//nodes/2.js'),
			() => import('.//nodes/3.js'),
			() => import('.//nodes/4.js')
		],
		routes: [
			{
				id: "todos",
				pattern: /^\/todos\/?$/,
				names: [],
				types: [],
				page: { layouts: [0], errors: [1], leaf: 4 },
				endpoint: null
			}
		],
		matchers: async () => {
			
			return {  };
		}
	}
};