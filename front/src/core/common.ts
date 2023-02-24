/**
 * This file was automatically generated from 'cargo types' command.
 * DO NOT MODIFY IT BY HAND. Instead, modify the source code in gpu-common,
 * and run 'cargo types' again to regenerate this file.
 */

export type Action =
	| {
			c: Pane
			ty: 'togglePanel'
	  }
	| {
			ty: 'toggleDebugPanel'
	  }
	| {
			ty: 'toggleUserPreferences'
	  }
	| {
			ty: 'playPause'
	  }
	| {
			ty: 'reset'
	  }
	| {
			ty: 'rebuild'
	  }
	| {
			ty: 'toggleConsole'
	  }
	| {
			c: Pane
			ty: 'focus'
	  }
	| {
			ty: 'closeDocument'
	  }
	| {
			ty: 'nextDocument'
	  }
	| {
			ty: 'previousDocument'
	  }
	| {
			c: string
			ty: 'openDocument'
	  }
	| {
			ty: 'createNewProject'
	  }
	| {
			ty: 'createNewFile'
	  }
	| {
			ty: 'saveProjectToRemote'
	  }
	| {
			ty: 'saveCurrentFile'
	  }
	| {
			ty: 'saveAllFiles'
	  }
	| {
			ty: 'fork'
	  }
	| {
			ty: 'publish'
	  }
	| {
			ty: 'closeFile'
	  }
	| {
			ty: 'closeProject'
	  }
	| {
			c: string
			ty: 'setRunner'
	  }
	| {
			/**
			 * @minItems 2
			 * @maxItems 2
			 */
			c: [string, string]
			ty: 'move'
	  }
	| {
			/**
			 * @minItems 2
			 * @maxItems 2
			 */
			c: [string, string]
			ty: 'copy'
	  }

export type Pane = 'projectPane' | 'editorPane' | 'resourcePane'

export type LogLevel = 'Trace' | 'Debug' | 'Info' | 'Warn' | 'Error'

export type PerformanceLevel = 'Default' | 'PowerSaver'

export interface Config {
	limitFps?: number
	logLevel?: LogLevel & string
	perfLevel?: PerformanceLevel | null
	runner?: string | null
}

export interface Credentials {
	password: string
	usernameOrEmail: string
}

export interface LoginResponse {
	userId: string
}

export interface NewUser {
	email: string
	password: string
	username: string
}

export interface NewUserResponse {
	id: string
}

export type SupportedExtension =
	| 'wgsl'
	| 'glsl'
	| 'txt'
	| 'md'
	| 'json'
	| 'csv'
	| 'png'
	| 'jpeg'
	| 'mp3'

export type Binding =
	| {
			Builtin: Builtin
	  }
	| {
			Location: {
				interpolation?: Interpolation | null
				location: number
				sampling?: Sampling | null
			}
	  }

export type Builtin =
	| (
			| 'ViewIndex'
			| 'BaseInstance'
			| 'BaseVertex'
			| 'ClipDistance'
			| 'CullDistance'
			| 'InstanceIndex'
			| 'PointSize'
			| 'VertexIndex'
			| 'FragDepth'
			| 'FrontFacing'
			| 'PrimitiveIndex'
			| 'SampleIndex'
			| 'SampleMask'
			| 'GlobalInvocationId'
			| 'LocalInvocationId'
			| 'LocalInvocationIndex'
			| 'WorkGroupId'
			| 'WorkGroupSize'
			| 'NumWorkGroups'
			| 'PointCoord'
	  )
	| {
			Position: {
				invariant: boolean
			}
	  }

export type Interpolation = 'Perspective' | 'Linear' | 'Flat'

export type Sampling = 'Center' | 'Centroid' | 'Sample'

export type ShaderStage = 'Vertex' | 'Fragment' | 'Compute'

export type TypeInner =
	| {
			Scalar: {
				kind: ScalarKind
				width: number
			}
	  }
	| {
			Vector: {
				kind: ScalarKind
				size: VectorSize
				width: number
			}
	  }
	| {
			Matrix: {
				colums: VectorSize
				rows: VectorSize
				width: number
			}
	  }
	| {
			Atomic: {
				kind: ScalarKind
				width: number
			}
	  }
	| {
			Pointer: {
				base: number
				space: AddressSpace
			}
	  }
	| {
			ValuePointer: {
				kind: ScalarKind
				size?: VectorSize | null
				space: AddressSpace
				width: number
			}
	  }
	| {
			Array: {
				base: number
				size: ArraySize
				stride: number
			}
	  }
	| {
			Struct: {
				members: StructMember[]
				span: number
			}
	  }
	| {
			Image: {
				arrayed: boolean
				class: ImageClass
				dim: ImageDimension
			}
	  }
	| {
			Sampler: {
				comparison: boolean
			}
	  }
	| {
			BindingArray: {
				base: number
				size: ArraySize
			}
	  }

export type ScalarKind = 'Sint' | 'Uint' | 'Float' | 'Bool'

export type VectorSize = 'Bi' | 'Tri' | 'Quad'

export type AddressSpace =
	| (
			| 'Function'
			| 'Private'
			| 'WorkGroup'
			| 'Uniform'
			| 'Handle'
			| 'PushConstant'
	  )
	| {
			Storage: {
				access: number
			}
	  }

export type ArraySize =
	| 'Dynamic'
	| {
			Constant: number
	  }

export type ImageClass =
	| {
			Sampled: {
				kind: ScalarKind
				multi: boolean
			}
	  }
	| {
			Depth: {
				multi: boolean
			}
	  }
	| {
			Storage: {
				access: number
				format: StorageFormat
			}
	  }

export type StorageFormat =
	| 'R8Unorm'
	| 'R8Snorm'
	| 'R8Uint'
	| 'R8Sint'
	| 'R16Uint'
	| 'R16Sint'
	| 'R16Float'
	| 'Rg8Unorm'
	| 'Rg8Snorm'
	| 'Rg8Uint'
	| 'Rg8Sint'
	| 'R32Uint'
	| 'R32Sint'
	| 'R32Float'
	| 'Rg16Uint'
	| 'Rg16Sint'
	| 'Rg16Float'
	| 'Rgba8Unorm'
	| 'Rgba8Snorm'
	| 'Rgba8Uint'
	| 'Rgba8Sint'
	| 'Rgb10a2Unorm'
	| 'Rg11b10Float'
	| 'Rg32Uint'
	| 'Rg32Sint'
	| 'Rg32Float'
	| 'Rgba16Uint'
	| 'Rgba16Sint'
	| 'Rgba16Float'
	| 'Rgba32Uint'
	| 'Rgba32Sint'
	| 'Rgba32Float'
	| 'R16Unorm'
	| 'R16Snorm'
	| 'Rg16Unorm'
	| 'Rg16Snorm'
	| 'Rgba16Unorm'
	| 'Rgba16Snorm'

export type ImageDimension = '1d' | '2d' | '3d' | 'cube'

export interface PrebuildResult {
	dependencyInfo: DependencyInfo
	fileBuilds: {
		[k: string]: FilePrebuildResult
	}
}

export interface DependencyInfo {
	deps: {
		[k: string]: FileDependencyInfo
	}
}

export interface FileDependencyInfo {
	errors?: CompileError[] | null
	exxports: {
		[k: string]: Match
	}
	imports: Match[]
}

export interface CompileError {
	message: string
	span?: SourceLocation | null
}

export interface SourceLocation {
	/**
	 * Length in code units (in bytes) of the span.
	 */
	length: number
	/**
	 * 1-based line number.
	 */
	lineNumber: number
	/**
	 * 1-based column of the start of this span
	 */
	linePosition: number
	/**
	 * 0-based Offset in code units (in bytes) of the start of the span.
	 */
	offset: number
}
/**
 * Identical to regex::Match, except the text is owned and it can be serialized. TODO: get refs to work within the analyzer instead of owned strings.
 */

export interface Match {
	end: number
	start: number
	text: string
}

export interface FilePrebuildResult {
	errors?: CompileError[] | null
	processedShader: File
	rawModule?: Module | null
}
/**
 * Encapsulates all data needed to emulate a file in gputoy virtual directory structure.
 */

export interface File {
	/**
	 * Contents of file in plain text
	 */
	data: string
	/**
	 * File path starting at / (project root)
	 */
	dir: string
	/**
	 * File extension
	 */
	extension: SupportedExtension
	/**
	 * Fetch url. If exists, then contents will be fetched from remote URL on project load
	 */
	fetch?: string | null
	/**
	 * Name of file
	 */
	fileName: string
}

export interface Module {
	entry_points: EntryPoint[]
	functions: Function[]
	types: Type[]
}

export interface EntryPoint {
	function: Function
	name: string
	stage: ShaderStage
	/**
	 * @minItems 3
	 * @maxItems 3
	 */
	workgroup_size: [number, number, number]
}

export interface Function {
	arguments: FunctionArgument[]
	name?: string | null
	result?: FunctionResult | null
}

export interface FunctionArgument {
	binding?: Binding | null
	name?: string | null
	ty: number
}

export interface FunctionResult {
	binding?: Binding | null
	ty: number
}

export interface Type {
	inner: TypeInner
	name?: string | null
}

export interface StructMember {
	binding?: Binding | null
	name?: string | null
	offset: number
	ty: number
}

export interface Project {
	config?: Config | null
	files: Files
	layout?: Layout | null
}

export interface Config {
	limitFps?: number
	logLevel?: LogLevel & string
	perfLevel?: PerformanceLevel | null
	runner?: string | null
}
/**
 * Gputoy virtual directory. Each file in the map has its path from root as key, including file name and extension
 *
 * example: ```ts map: { "/shaders/main.wgsl": { "data": "...", "dir": "shaders/", "fileName": "main", "extension": "wgsl", } } ```
 */

export interface Files {
	map: {
		[k: string]: File
	}
}
/**
 * Encapsulates all data needed to emulate a file in gputoy virtual directory structure.
 */

export interface Layout {
	/**
	 * State of project panel accordians
	 */
	accordianOpen: {
		[k: string]: boolean
	}
	/**
	 * Currently opened file index within workspace
	 */
	fileIndex?: number | null
	/**
	 * State of file tree
	 */
	fileTreeState: {
		[k: string]: DirNodeState
	}
	/**
	 * Pane size data
	 */
	paneSize: PaneSize
	/**
	 * Pane toggle data
	 */
	paneToggled: PaneToggled
	/**
	 * List of file identifiers which is open in workspace. Order of identifiers in vec is the order it is listed in the editor.
	 */
	workspace: string[]
}

export interface DirNodeState {
	isRenaming: boolean
	open: boolean
}
/**
 * Pane size information.
 *
 * Persistent layout data needed to give 'preferred size' to panes.
 */

export interface PaneSize {
	/**
	 * What percentage of total window width the editor pane takes up.
	 *
	 * It is assumed the viewport/resource pane will take up the remaining space left behind by the project and editor pane.
	 */
	editorPanePercentage: number
	/**
	 * How many pixels wide the project pane should be.
	 *
	 * Will change its percentage share of the window if window width changes.
	 */
	projectPanePx: number
	/**
	 * What percentage of total window height the resource pane takes up.
	 */
	resourcePanePercentage: number
}
/**
 * Pane layout information.
 */

export interface PaneToggled {
	/**
	 * Whether the editor pane is open
	 */
	editorPane: boolean
	/**
	 * Whether the project pane is open
	 */
	projectPane: boolean
	/**
	 * Whether the resource pane is open
	 */
	resourcePane: boolean
}

export interface ProjectResponse {
	authorId?: string | null
	config?: Config | null
	createdAt: string
	description?: string | null
	files: Files
	forkedFromId?: string | null
	id: string
	layout?: Layout | null
	published: boolean
	title: string
	updatedAt: string
}

export interface ProjectUpsert {
	config?: Config | null
	description?: string | null
	files: Files
	id?: string | null
	layout?: Layout | null
	published: boolean
	title: string
}

export type BundleArgs = {
	/**
	 * Target canvas id
	 */
	target: string
	type: 'Viewport'
}

export type PipelineArgs =
	| VertexFragmentPipeline
	| FullscreenQuadPipeline
	| ComputePipeline
/**
 * A path to "wgsl" or "glsl" shader file that contains a fragment entry point.
 *
 * Note: this may be the same as vertex shader as long as that file has both vertex and fragment entry points.
 */

export type FragmentShader = string
/**
 * Ordered list of texture resources the fragment shader will draw to. Ordering will correspond to location attributes in shader output.
 */

export type FragmentShaderTargets = string[]
/**
 * A path to "wgsl" or "glsl" shader file that contains a vertex entry point.
 *
 * Note: this may be the same as fragment shader as long as that file has both vertex and fragment entry points.
 */

export type VertexShader = string
/**
 * A path to "wgsl" or "glsl" shader file that contains a fragment entry point.
 */

export type FragmentShader1 = string
/**
 * Ordered list of texture resources the fragment shader will draw to. Ordering will correspond to location attributes in shader output.
 */

export type FragmentShaderTargets1 = string[]
/**
 * A path to "wgsl" or "glsl" shader file that contains a compute entry point.
 */

export type ComputeShader = string
/**
 * A pipeline represents an execution of shader.
 */

export type RunnerPipelines = PipelineArgs[]

/**
 * A runner is used to orchestrate shader execution and resource management.
 *
 * A project can have multiple runners, but will default to /run.json.
 */

export interface Runner {
	bundles: RunnerBundles
	pipelines: RunnerPipelines
}
/**
 * Bundles are a collection of Resources surrounding some form of input/ouput that are maintained around a runner's lifecycle (run start, run end, frame start, frame end).
 *
 * For example, the Viewport bundle has a 'surface' texture resource which can be written to in fragment shader, a 'mouse' uniform buffer corresponding to mouse position over said viewport, and a 'resolution' uniform buffer that contains the current resolution of the viewport.
 *
 * In the pipelines, these resources can be used just like a resource defined by the user. Only instead of 'res::{some_resource_name}', the identifier will be '{some_bundle_name}::{some_resource_within_bundle}'.
 *
 * For instance, if you wanted to use the surface of a Viewport bundle named 'view' within a pipeline, you would identiy it like 'view::surface'.
 */

export interface RunnerBundles {
	[k: string]: BundleArgs
}
/**
 * Plain rasterizer pipeline with configurable vertex inputs and texture output targets.
 *
 * Note: While this can be used for fullscreen quad, the easier method would to be to use the built in FullscreenQuad pipeline, which handles the vertex shader automatically.
 */

export interface VertexFragmentPipeline {
	VertexFragement: VertexFragment
}

export interface VertexFragment {
	binds?: PipelineBinds | null
	fragment: FragmentShader
	targets: FragmentShaderTargets
	vertex: VertexShader
}
/**
 * Describes a map between syned shader variables and resource path.
 *
 * During build, the resource at named sync variable will be bound via bind groups.
 *
 * Resources can be from either project resources or from a bundle. For example, "resource::particles" will look for defined resource named "particles", and "view::surface" will look for the "surface" resource from defined bundle called "view".
 */

export interface PipelineBinds {}
/**
 * Fragment shader over rasterized fullscreen quad.
 *
 * Similar in function to a 'shadertoy' shader.
 */

export interface FullscreenQuadPipeline {
	FullscreenQuad: FullscreenQuad
}

export interface FullscreenQuad {
	binds?: PipelineBinds | null
	fragment: FragmentShader1
	targets: FragmentShaderTargets1
}
/**
 * Compute pipeline to be ran over the range of some resource.
 */

export interface ComputePipeline {
	Compute: Compute
}

export interface Compute {
	binds?: PipelineBinds | null
	shader: ComputeShader
}

export type LineNumberPrefs = 'on' | 'interval' | 'relative' | 'off'

export interface UpdateUserInfoArgs {
	bio?: string | null
	config?: UserPrefs | null
	fullName?: string | null
	image?: string | null
}

export interface UserPrefs {
	editor: UserEditorPrefs
	general: UserGeneralPrefs
	keybinds: {
		[k: string]: FilteredAction
	}
	theme: {
		[k: string]: string
	}
}

export interface UserEditorPrefs {
	fontFamily?: string | null
	fontSize?: number | null
	lineNumbers: LineNumberPrefs
	minimap: boolean
	vimMode: boolean
}

export interface UserGeneralPrefs {
	consoleWrap: boolean
	editorPanelSize: number
	projectPanelSize: number
	resourcePanelSize: number
}

export interface FilteredAction {
	action: Action
	condition?: string | null
}

export interface UserInfoResponse {
	active: boolean
	bio?: string | null
	config?: UserPrefs | null
	createdAt: string
	email: string
	emailVerified: boolean
	fullName?: string | null
	id: string
	image?: string | null
	updatedAt: string
	username: string
}
