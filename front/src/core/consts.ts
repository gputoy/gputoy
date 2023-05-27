import type { Keybinds } from '$core/keys'
import type { RunState } from '$core/runstate'
import type { Action, Config, DirNodeState, Files, Layout } from '$gen'

/**
 *              Points to gpu-front (this)
 */
export const BASE_URL = import.meta.env.VITE_FE_URL + '/'

/**
 *              Points to gpu-back
 */
export const API_URL = import.meta.env.VITE_API_URL + '/'

export const WASM_CLIENT_URL = BASE_URL + import.meta.env.VITE_MAKE_CLIENT_PATH
export const WASM_COMMON_URL = BASE_URL + import.meta.env.VITE_MAKE_COMMON_PATH
export const WASM_ANALYZER_URL =
	BASE_URL + import.meta.env.VITE_MAKE_ANALYZER_PATH

export const DEFAULT_RUN_STATE: RunState = {
	playing: false
}

const main = `// shader by toto https://www.shadertoy.com/view/wlVGWd
// converted to wgsl

fn rand(n: vec2<f32>) -> f32 {
    return fract(sin(dot(n, vec2<f32>(12.9898, 4.1414))) * 43758.5453);
}

fn noise(p: vec2<f32>) -> f32 {
    let ip: vec2<f32> = floor(p);
    var u: vec2<f32> = fract(p);
    u = u*u*(3.0-(2.0*u));

    let res: f32 = mix(
        mix(rand(ip),rand(ip+vec2<f32>(1.0,0.0)),u.x),
        mix(rand(ip+vec2<f32>(0.0,1.0)),rand(ip+vec2<f32>(1.0,1.0)),u.x),u.y);
    return res*res;
}

fn fbm(qIn: vec2<f32>) -> f32{
    let m2: mat2x2<f32> = mat2x2<f32>(vec2<f32>(0.8,-0.6), vec2<f32>(0.6,0.8));
    var q: vec2<f32> = qIn;
    var f: f32 = 0.0;
    f = f + 0.5000*noise( q ); q = m2*q*2.02;
    f = f + 0.2500*noise( q ); q = m2*q*2.03;
    f = f + 0.1250*noise( q ); q = m2*q*2.01;
    f = f + 0.0625*noise( q );

    return f/0.769;
}

fn pattern(s: vec2<f32> ) -> f32 {
  let q: vec2<f32> = vec2<f32>(fbm(s + vec2<f32>(0.0,0.0)));
  var r: vec2<f32> = vec2<f32>(fbm(s + 4.0*q + vec2<f32>(1.7,9.2)));
  r = r + i.time * p.speed * 0.15;
  return fbm( s + 1.760*r );
}

[[stage(fragment)]]
fn main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    
    let scaled: vec2<f32> = in.uv * 4.5; // Scale UV to make it nicer in that big screen !
    let displacement: f32 = pattern(scaled);
    var color: vec4<f32> = vec4<f32>(displacement * 1.2, 0.2, displacement * 5., 1.);
    color = color * vec4<f32>(p.colorMod, 1.0);
    return color;

}
`

/**
 *              Project defaults
 */
export const DEFAULT_DIR_NODE_STATE: DirNodeState = {
	open: false,
	isRenaming: false
}
export const DEFAULT_LAYOUT: Layout = {
	tabIndex: 0,
	tabs: ['/shaders/main.wgsl', '/run.json'] as string[],
	paneSize: {
		projectPanePx: 185,
		editorPanePercentage: 40,
		controlPanePercentage: 38
	},
	paneToggled: {
		projectPane: true,
		editorPane: true,
		controlPane: true
	},
	fileTreeState: {
		'/shaders': {
			...DEFAULT_DIR_NODE_STATE,
			open: true
		}
	},
	accordianOpen: {
		Summary: true,
		Files: true
	}
} as const
export const DEFAULT_CONFIG: Config = {
	runner: '/run.json'
} as const
export const DEFAULT_FILES: Files = {
	map: {
		'/shaders/main.wgsl': {
			data: main,
			dir: 'shaders',
			fileName: 'main',
			extension: 'wgsl'
		},
		'/run.json': {
			data: '...',
			dir: '',
			fileName: 'run',
			extension: 'json'
		}
	}
} as const

export const DEFAULT_USER_KEYBINDS: Keybinds = {
	'C-g': {
		action: {
			ty: 'toggleUi',
			c: 'terminal'
		}
	},
	'A-t': {
		action: {
			ty: 'toggleUi',
			c: 'terminal'
		}
	},
	'C-q': {
		action: {
			ty: 'toggleUi',
			c: 'projectPane'
		}
	},
	'C-e': {
		action: {
			ty: 'toggleUi',
			c: 'editorPane'
		}
	},
	'C-r': {
		action: {
			ty: 'toggleUi',
			c: 'controlPane'
		}
	},
	'C-S-d': {
		action: {
			ty: 'toggleUi',
			c: 'debug'
		}
	},
	'C-a': {
		action: {
			ty: 'toggleAllPanes'
		}
	},
	'C-j': {
		action: {
			ty: 'prevTab'
		}
	},
	'C-k': {
		action: {
			ty: 'nextTab'
		}
	},
	'C-s': {
		action: {
			ty: 'saveFile'
		}
	},
	'C-u': {
		action: {
			ty: 'closeTab'
		}
	},
	'C-.': {
		action: {
			ty: 'toggleUi',
			c: 'preferences'
		}
	}
} as const

/**
 *                  Nav UI Menu
 */
export const MENUKEYS = ['file', 'edit', 'project', 'view', 'help'] as const
export type MenuKey = (typeof MENUKEYS)[number]
export type MenuEntry = {
	name: string
	fAction?: {
		action: Action
	}
}

export const MENU_MAP: Record<
	MenuKey,
	ReadonlyArray<ReadonlyArray<MenuEntry>>
> = {
	file: [
		[
			{
				name: 'New Project',
				fAction: {
					action: {
						ty: 'newProject',
						c: ''
					}
				}
			},
			{
				name: 'New File',
				fAction: {
					action: {
						ty: 'newFile',
						c: ''
					}
				}
			}
		],
		[
			{
				name: 'Commit',
				fAction: {
					action: {
						ty: 'commit'
					}
					// condition: 'userLoggedIn'
				}
			},
			{
				name: 'Save',
				fAction: {
					action: {
						ty: 'saveFile'
					}
					// condition: 'currentFileDirty'
				}
			},
			{
				name: 'Save all',
				fAction: {
					action: {
						ty: 'saveAllFiles'
					}
					// condition: 'fileDirty'
				}
			}
		],
		[
			{
				name: 'Fork',
				fAction: {
					action: {
						ty: 'fork',
						c: ''
					}
				}
			},
			{
				name: 'Publish',
				fAction: {
					action: {
						ty: 'publish'
					}
					// condition: 'userLoggedIn'
				}
			}
		],
		[
			{
				name: 'Preferences',
				fAction: {
					action: {
						ty: 'toggleUi',
						c: 'preferences'
					}
				}
			}
		],
		[
			{
				name: 'Close file',
				fAction: {
					action: {
						ty: 'closeTab'
					}
				}
			},

			{
				name: 'Exit',
				fAction: {
					action: {
						ty: 'exit'
					}
				}
			}
		]
	],
	edit: [],
	project: [
		[
			{
				name: 'Build',
				fAction: {
					action: {
						ty: 'build'
					}
				}
			},
			{
				name: 'Introspect',
				fAction: {
					action: {
						ty: 'toggleUi',
						c: 'terminal'
					}
				}
			}
		]
	],
	view: [
		[
			{
				name: 'Toggle Project Pane',
				fAction: {
					action: {
						ty: 'toggleUi',
						c: 'projectPane'
					}
				}
			},
			{
				name: 'Toggle Editor Pane',
				fAction: {
					action: {
						ty: 'toggleUi',
						c: 'editorPane'
					}
				}
			},
			{
				name: 'Toggle Control Pane',
				fAction: {
					action: {
						ty: 'toggleUi',
						c: 'controlPane'
					}
				}
			}
		],
		[
			{
				name: 'Toggle Terminal',
				fAction: {
					action: {
						ty: 'toggleUi',
						c: 'terminal'
					}
				}
			},
			{
				name: 'Toggle Preferences',
				fAction: {
					action: {
						ty: 'toggleUi',
						c: 'preferences'
					}
				}
			},
			{
				name: 'Toggle User Info',
				fAction: {
					action: {
						ty: 'toggleUi',
						c: 'user'
					}
				}
			},
			{
				name: 'Toggle Debug',
				fAction: {
					action: {
						ty: 'toggleUi',
						c: 'debug'
					}
				}
			}
		],
		[
			// {
			//     name: 'Toggle Dark Mode',
			//     fAction: {
			//         action: {
			//             'ty': 'toggleDarkMode'
			//         }
			//     }
			// }
		]
	],
	help: []
} as const
