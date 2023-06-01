export const PREFERENCE_KEYS = [
    'workspace.handle-size',
    'workspace.ui-speed',
    'editor.auto-indent',
    'editor.line-numbers',
    'editor.font',
    'editor.font-size',
    'editor.font-ligatures',
    'editor.smooth-scrolling',
    'editor.smooth-caret',
    'editor.cursor-style',
    'editor.cursor-blinking',
    'editor.word-wrap',
    'editor.scroll-beyond-last-line',
    'editor.minimap',
    'editor.vim-mode.some-property',
    'console.show-completions',
    'console.wrap',
    'console.level',
] as const
export type PreferenceKey = typeof PREFERENCE_KEYS[number]

export enum CompletionIndex {
	Empty = 0,
	ActionKey,
	Str,
	FilePath,
	Path,
	Resource,
	PreferenceKey,
	Region,
	Key,
	AutoIndent,
	CursorBlinking,
	CursorStyle,
	LineNumbers,
	LogLevel,
	ShowCompletions,
	UiSpeed,
}
export const COMPLETION_KEY_TO_INDEX: Record<CompletionKey, CompletionIndex> = {
	'Empty': CompletionIndex.Empty,
	'ActionKey': CompletionIndex.ActionKey,
	'Str': CompletionIndex.Str,
	'FilePath': CompletionIndex.FilePath,
	'Path': CompletionIndex.Path,
	'Resource': CompletionIndex.Resource,
	'PreferenceKey': CompletionIndex.PreferenceKey,
	'Region': CompletionIndex.Region,
	'Key': CompletionIndex.Key,
	'AutoIndent': CompletionIndex.AutoIndent,
	'CursorBlinking': CompletionIndex.CursorBlinking,
	'CursorStyle': CompletionIndex.CursorStyle,
	'LineNumbers': CompletionIndex.LineNumbers,
	'LogLevel': CompletionIndex.LogLevel,
	'ShowCompletions': CompletionIndex.ShowCompletions,
	'UiSpeed': CompletionIndex.UiSpeed,
} as const

/**
 * This file was automatically generated from 'cargo types' command.
 * DO NOT MODIFY IT BY HAND. Instead, modify the source code in gpu-common,
 * and run 'cargo types' again to regenerate this file.
 */

export type SupportedExtension = "wgsl" | "glsl" | "txt" | "md" | "json" | "csv" | "png" | "jpeg" | "mp3";

export type PerformanceLevel = "Default" | "PowerSaver";

export type FilePath = string;
/**
 * The speed of ui transitions.
 */

export type UiSpeed = "instant" | "snappy" | "quick" | "smooth" | "slow";
/**
 * Controls whether the editor should automatically adjust the indentation when users type, paste, move or indent lines.
 */

export type AutoIndent = "none" | "keep" | "brackets" | "advanced" | "full";
/**
 * How line numbers appear in the code editor.
 */

export type LineNumbers = "on" | "interval" | "relative" | "off";
/**
 * Style of the cursor in the code editor.
 */

export type CursorStyle = "line" | "line-thin" | "block" | "block-outline" | "underline" | "underline-thin";
/**
 * Controls the cursor animation style in the code editor.
 */

export type CursorBlinking = "blink" | "smooth" | "phase" | "expand" | "solid";
/**
 * How much detail should completions provide.
 */

export type ShowCompletions = "full" | "suggestions" | "none";
/**
 * Log level of the console.
 */

export type LogLevel = "trace" | "debug" | "info" | "warn" | "error";

export type Action =
  | {
      ty: "clear";
    }
  | {
      ty: "show";
      c: Region;
    }
  | {
      ty: "hide";
      c: Region;
    }
  | {
      ty: "toggleUi";
      c: Region;
    }
  | {
      ty: "toggleAllPanes";
    }
  | {
      ty: "bindKey";
      c: BindKey;
    }
  | {
      ty: "toggleDebugPanel";
    }
  | {
      ty: "openTab";
      c: FilePath;
    }
  | {
      ty: "closeTab";
    }
  | {
      ty: "nextTab";
    }
  | {
      ty: "prevTab";
    }
  | {
      ty: "newFile";
      c: FilePath;
    }
  | {
      ty: "newDir";
      c: Path;
    }
  | {
      ty: "move";
      c: CopyMove;
    }
  | {
      ty: "copy";
      c: CopyMove;
    }
  | {
      ty: "delete";
      c: Delete;
    }
  | {
      ty: "saveFile";
    }
  | {
      ty: "saveAllFiles";
    }
  | {
      ty: "newProject";
      c: string | null;
    }
  | {
      ty: "commit";
    }
  | {
      ty: "fork";
      c: string | null;
    }
  | {
      ty: "publish";
    }
  | {
      ty: "setRunner";
      c: FilePath;
    }
  | {
      ty: "playPause";
    }
  | {
      ty: "reset";
    }
  | {
      ty: "build";
    }
  | {
      ty: "exit";
    };
/**
 * A UI region within the workspace
 */

export type Region = "project-pane" | "control-pane" | "editor-pane" | "preferences" | "debug" | "terminal" | "user";

export type Key = string;

export type Path = string;

export type TypeInner =
  | {
      Scalar: {
        kind: ScalarKind;
        width: number;
      };
    }
  | {
      Vector: {
        size: VectorSize;
        kind: ScalarKind;
        width: number;
      };
    }
  | {
      Matrix: {
        colums: VectorSize;
        rows: VectorSize;
        width: number;
      };
    }
  | {
      Atomic: {
        kind: ScalarKind;
        width: number;
      };
    }
  | {
      Pointer: {
        base: number;
        space: AddressSpace;
      };
    }
  | {
      ValuePointer: {
        size?: VectorSize | null;
        kind: ScalarKind;
        width: number;
        space: AddressSpace;
      };
    }
  | {
      Array: {
        base: number;
        size: ArraySize;
        stride: number;
      };
    }
  | {
      Struct: {
        members: StructMember[];
        span: number;
      };
    }
  | {
      Image: {
        dim: ImageDimension;
        arrayed: boolean;
        class: ImageClass;
      };
    }
  | {
      Sampler: {
        comparison: boolean;
      };
    }
  | {
      BindingArray: {
        base: number;
        size: ArraySize;
      };
    };

export type ScalarKind = "Sint" | "Uint" | "Float" | "Bool";

export type VectorSize = "Bi" | "Tri" | "Quad";

export type AddressSpace =
  | ("Function" | "Private" | "WorkGroup" | "Uniform" | "Handle" | "PushConstant")
  | {
      Storage: {
        access: number;
      };
    };

export type ArraySize =
  | "Dynamic"
  | {
      Constant: number;
    };

export type Binding =
  | {
      Builtin: Builtin;
    }
  | {
      Location: {
        location: number;
        interpolation?: Interpolation | null;
        sampling?: Sampling | null;
      };
    };

export type Builtin =
  | (
      | "ViewIndex"
      | "BaseInstance"
      | "BaseVertex"
      | "ClipDistance"
      | "CullDistance"
      | "InstanceIndex"
      | "PointSize"
      | "VertexIndex"
      | "FragDepth"
      | "FrontFacing"
      | "PrimitiveIndex"
      | "SampleIndex"
      | "SampleMask"
      | "GlobalInvocationId"
      | "LocalInvocationId"
      | "LocalInvocationIndex"
      | "WorkGroupId"
      | "WorkGroupSize"
      | "NumWorkGroups"
      | "PointCoord"
    )
  | {
      Position: {
        invariant: boolean;
      };
    };

export type Interpolation = "Perspective" | "Linear" | "Flat";

export type Sampling = "Center" | "Centroid" | "Sample";

export type ImageDimension = "1d" | "2d" | "3d" | "cube";

export type ImageClass =
  | {
      Sampled: {
        kind: ScalarKind;
        multi: boolean;
      };
    }
  | {
      Depth: {
        multi: boolean;
      };
    }
  | {
      Storage: {
        format: StorageFormat;
        access: number;
      };
    };

export type StorageFormat =
  | "R8Unorm"
  | "R8Snorm"
  | "R8Uint"
  | "R8Sint"
  | "R16Uint"
  | "R16Sint"
  | "R16Float"
  | "Rg8Unorm"
  | "Rg8Snorm"
  | "Rg8Uint"
  | "Rg8Sint"
  | "R32Uint"
  | "R32Sint"
  | "R32Float"
  | "Rg16Uint"
  | "Rg16Sint"
  | "Rg16Float"
  | "Rgba8Unorm"
  | "Rgba8Snorm"
  | "Rgba8Uint"
  | "Rgba8Sint"
  | "Rgb10a2Unorm"
  | "Rg11b10Float"
  | "Rg32Uint"
  | "Rg32Sint"
  | "Rg32Float"
  | "Rgba16Uint"
  | "Rgba16Sint"
  | "Rgba16Float"
  | "Rgba32Uint"
  | "Rgba32Sint"
  | "Rgba32Float"
  | "R16Unorm"
  | "R16Snorm"
  | "Rg16Unorm"
  | "Rg16Snorm"
  | "Rgba16Unorm"
  | "Rgba16Snorm";

export type ShaderStage = "Vertex" | "Fragment" | "Compute";

export type BundleArgs = {
  type: "Viewport";
  /**
   * Target canvas id
   */
  target: string;
};

export type PipelineArgs = VertexFragmentPipeline | FullscreenQuadPipeline | ComputePipeline;
/**
 * A path to "wgsl" or "glsl" shader file that contains a vertex entry point.
 *
 * Note: this may be the same as fragment shader as long as that file has both vertex and fragment entry points.
 */

export type VertexShader = FilePath;
/**
 * A path to "wgsl" or "glsl" shader file that contains a fragment entry point.
 *
 * Note: this may be the same as vertex shader as long as that file has both vertex and fragment entry points.
 */

export type FragmentShader = FilePath;
/**
 * Ordered list of texture resources the fragment shader will draw to. Ordering will correspond to location attributes in shader output.
 */

export type FragmentShaderTargets = string[];
/**
 * A path to "wgsl" or "glsl" shader file that contains a fragment entry point.
 */

export type FragmentShader1 = FilePath;
/**
 * Ordered list of texture resources the fragment shader will draw to. Ordering will correspond to location attributes in shader output.
 */

export type FragmentShaderTargets1 = string[];
/**
 * A path to "wgsl" or "glsl" shader file that contains a compute entry point.
 */

export type ComputeShader = FilePath;
/**
 * A pipeline represents an execution of shader.
 */

export type RunnerPipelines = PipelineArgs[];

export type Severity = "error" | "warning";

export type Destination = "console" | "toast";
/**
 * Describes the various classes of user interactables
 *
 * View `front/src/components/preferences/*Controller.svelte` for more info on how these input classes are implemented in the frontend.
 */

export type ConfigValueClass =
  | {
      ty: "IntClass";
      c: Int;
    }
  | {
      ty: "FloatClass";
      c: FloatInput;
    }
  | {
      ty: "StrClass";
      c: StrClass;
    }
  | {
      ty: "EnumClass";
      c: EnumClass;
    }
  | {
      ty: "BoolClass";
      c: BoolClass;
    }
  | {
      ty: "CategoryClass";
      c: CategoryClass;
    }
  | {
      ty: "ToggledCategoryClass";
      c: CategoryClass;
    }
  | {
      ty: "CmdClass";
      c: CmdClass;
    };

export type CompletionKey =
  | ("AutoIndent" | "CursorBlinking" | "CursorStyle" | "LineNumbers" | "LogLevel" | "ShowCompletions" | "UiSpeed")
  | "Empty"
  | "ActionKey"
  | "Str"
  | "FilePath"
  | "Path"
  | "Resource"
  | "PreferenceKey"
  | "Region"
  | "Key";

export interface RootSchema {
  project: Project;
  config: Config;
  projectupsert: ProjectUpsert;
  projectresponse: ProjectResponse;
  newuser: NewUser;
  newuserresponse: NewUserResponse;
  credentials: Credentials;
  loginresponse: LoginResponse;
  userinforesponse: UserInfoResponse;
  updateuserinfoargs: UpdateUserInfoArgs;
  action: Action;
  prebuildresult: PrebuildResult;
  runner: Runner;
  clienterror: ClientError;
  preferences: Preferences;
  configvalueschema: ConfigValueSchema;
  completioninfo: CompletionInfo;
  completionentry: CompletionEntry;
}

export interface Project {
  files: Files;
  layout?: Layout | null;
  config?: Config | null;
}
/**
 * Gputoy virtual directory. Each file in the map has its path from root as key, including file name and extension
 *
 * example: ```ts map: { "/shaders/main.wgsl": { "data": "...", "dir": "shaders/", "fileName": "main", "extension": "wgsl", } } ```
 */

export interface Files {
  map: {
    [k: string]: File;
  };
}
/**
 * Encapsulates all data needed to emulate a file in gputoy virtual directory structure.
 */

export interface File {
  /**
   * Contents of file in plain text
   */
  data: string;
  /**
   * File path starting at / (project root)
   */
  dir: string;
  /**
   * Name of file
   */
  fileName: string;
  /**
   * File extension
   */
  extension: SupportedExtension;
  /**
   * Fetch url. If exists, then contents will be fetched from remote URL on project load
   */
  fetch?: string | null;
}

export interface Layout {
  /**
   * List of files which is open in editor. Order of identifiers in vec is the order it is listed in the editor.
   */
  tabs: string[];
  /**
   * Currently opened tab within workspace
   */
  tabIndex?: number | null;
  /**
   * Pane toggle data
   */
  paneToggled: PaneToggled;
  /**
   * Pane size data
   */
  paneSize: PaneSize;
  /**
   * State of file tree
   */
  fileTreeState: {
    [k: string]: DirNodeState;
  };
  /**
   * State of project panel accordians
   */
  accordianOpen: {
    [k: string]: boolean;
  };
}
/**
 * Pane layout information.
 */

export interface PaneToggled {
  /**
   * Whether the project pane is open
   */
  "project-pane": boolean;
  /**
   * Whether the editor pane is open
   */
  "editor-pane": boolean;
  /**
   * Whether the control pane is open
   */
  "control-pane": boolean;
}
/**
 * Pane size information.
 *
 * Persistent layout data needed to give 'preferred size' to panes.
 */

export interface PaneSize {
  /**
   * How many pixels wide the project pane should be.
   *
   * Will change its percentage share of the window if window width changes.
   */
  projectPanePx: number;
  /**
   * What percentage of total window width the editor pane takes up.
   *
   * It is assumed the viewport/resource pane will take up the remaining space left behind by the project and editor pane.
   */
  editorPanePercentage: number;
  /**
   * What percentage of total window height the control pane takes up.
   */
  controlPanePercentage: number;
}

export interface DirNodeState {
  open: boolean;
  isRenaming: boolean;
}

export interface Config {
  perfLevel?: PerformanceLevel | null;
  limitFps?: number;
  runner?: FilePath | null;
}

export interface ProjectUpsert {
  id?: string | null;
  title: string;
  description?: string | null;
  files: Files;
  layout?: Layout | null;
  config?: Config | null;
  published: boolean;
}

export interface ProjectResponse {
  id: string;
  title: string;
  description?: string | null;
  files: Files;
  layout?: Layout | null;
  config?: Config | null;
  published: boolean;
  createdAt: string;
  updatedAt: string;
  authorId?: string | null;
  forkedFromId?: string | null;
}

export interface NewUser {
  username: string;
  email: string;
  password: string;
}

export interface NewUserResponse {
  id: string;
}

export interface Credentials {
  usernameOrEmail: string;
  password: string;
}

export interface LoginResponse {
  userId: string;
}

export interface UserInfoResponse {
  id: string;
  username: string;
  email: string;
  fullName?: string | null;
  bio?: string | null;
  image?: string | null;
  emailVerified: boolean;
  preferences?: Preferences | null;
  active: boolean;
  createdAt: string;
  updatedAt: string;
}
/**
 * User preferences
 */

export interface Preferences {
  /**
   * Workspace preferences
   */
  workspace: Workspace;
  /**
   * Code editor preferences
   */
  editor: Editor;
  /**
   * Console preferences
   */
  console: Console;
}

export interface Workspace {
  /**
   * The size of the pane drag handle.
   */
  "handle-size": number;
  "ui-speed": UiSpeed;
}
/**
 * User editor preferences
 */

export interface Editor {
  "auto-indent": AutoIndent;
  "line-numbers": LineNumbers;
  /**
   * Font to use in the editor.
   *
   * Comma-delimited list like "FiraMono,mono"
   */
  font: string;
  /**
   * Font size of code in the editor.
   */
  "font-size": number;
  /**
   * Render font ligatures in the code editor.
   */
  "font-ligatures": boolean;
  /**
   * Animate smooth scrolling in the editor.
   */
  "smooth-scrolling": boolean;
  /**
   * Enable smooth caret movement animations.
   */
  "smooth-caret": boolean;
  "cursor-style": CursorStyle;
  "cursor-blinking": CursorBlinking;
  /**
   * Whether to wrap lines in the code editor.
   */
  "word-wrap": boolean;
  /**
   * Enable scrolling to go one screen size past the last line.
   */
  "scroll-beyond-last-line": boolean;
  /**
   * Whether to show a minimap in the top-right of the code editor.
   */
  minimap: boolean;
  "vim-mode": VimMode;
}
/**
 * Enable vim movements in code editor.
 */

export interface VimMode {
  enabled: boolean;
  "some-property": number;
}

export interface Console {
  "show-completions": ShowCompletions;
  /**
   * True: Wrap console messages. May scramble certain error messages. False: Console messages overflow past the size of the viewport.
   */
  wrap: boolean;
  level: LogLevel;
}

export interface UpdateUserInfoArgs {
  fullName?: string | null;
  bio?: string | null;
  image?: string | null;
  preferences?: Preferences | null;
}

export interface BindKey {
  key: Key;
  command: Action;
}

export interface CopyMove {
  src: Path;
  dest: Path;
  isDir: boolean;
}

export interface Delete {
  path: Path;
  isDir: boolean;
}

export interface PrebuildResult {
  dependencyInfo: DependencyInfo;
  fileBuilds: {
    [k: string]: FilePrebuildResult;
  };
}

export interface DependencyInfo {
  deps: {
    [k: string]: FileDependencyInfo;
  };
}

export interface FileDependencyInfo {
  imports: Match[];
  fileExports: {
    [k: string]: Match;
  };
  errors?: CompileError[] | null;
}
/**
 * Identical to regex::Match, except the text is owned and it can be serialized. TODO: get refs to work within the analyzer instead of owned strings.
 */

export interface Match {
  text: string;
  start: number;
  end: number;
}

export interface CompileError {
  message: string;
  span?: SourceLocation | null;
}

export interface SourceLocation {
  /**
   * 1-based line number.
   */
  lineNumber: number;
  /**
   * 1-based column of the start of this span
   */
  linePosition: number;
  /**
   * 0-based Offset in code units (in bytes) of the start of the span.
   */
  offset: number;
  /**
   * Length in code units (in bytes) of the span.
   */
  length: number;
}

export interface FilePrebuildResult {
  processedShader: File;
  rawModule?: Module | null;
  errors?: CompileError[] | null;
}

export interface Module {
  types: Type[];
  functions: Function[];
  entry_points: EntryPoint[];
}

export interface Type {
  name?: string | null;
  inner: TypeInner;
}

export interface StructMember {
  name?: string | null;
  ty: number;
  binding?: Binding | null;
  offset: number;
}

export interface Function {
  name?: string | null;
  arguments: FunctionArgument[];
  result?: FunctionResult | null;
}

export interface FunctionArgument {
  name?: string | null;
  ty: number;
  binding?: Binding | null;
}

export interface FunctionResult {
  ty: number;
  binding?: Binding | null;
}

export interface EntryPoint {
  name: string;
  stage: ShaderStage;
  /**
   * @minItems 3
   * @maxItems 3
   */
  workgroup_size: [number, number, number];
  function: Function;
}
/**
 * A runner is used to orchestrate shader execution and resource management.
 *
 * A project can have multiple runners, but will default to /run.json.
 */

export interface Runner {
  bundles: RunnerBundles;
  pipelines: RunnerPipelines;
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
  [k: string]: BundleArgs;
}
/**
 * Plain rasterizer pipeline with configurable vertex inputs and texture output targets.
 *
 * Note: While this can be used for fullscreen quad, the easier method would to be to use the built in FullscreenQuad pipeline, which handles the vertex shader automatically.
 */

export interface VertexFragmentPipeline {
  VertexFragement: VertexFragment;
}

export interface VertexFragment {
  vertex: VertexShader;
  fragment: FragmentShader;
  binds?: PipelineBinds | null;
  targets: FragmentShaderTargets;
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
  FullscreenQuad: FullscreenQuad;
}

export interface FullscreenQuad {
  fragment: FragmentShader1;
  binds?: PipelineBinds | null;
  targets: FragmentShaderTargets1;
}
/**
 * Compute pipeline to be ran over the range of some resource.
 */

export interface ComputePipeline {
  Compute: Compute;
}

export interface Compute {
  shader: ComputeShader;
  binds?: PipelineBinds | null;
}

export interface ClientError {
  severity: Severity;
  message: string;
  source: string;
  destination: Destination;
}

export interface ConfigValueSchema {
  name: string;
  description: string;
  path: string;
  class: ConfigValueClass;
}
/**
 * Describes the constraints of an int interactable.
 *
 * Used to generate html markup for changing fields of config structs.
 */

export interface Int {
  /**
   * The minimum value this input can be
   */
  min?: number | null;
  /**
   * The maximum value this input can be
   */
  max?: number | null;
  /**
   * The amount this input will step up and down
   */
  step: number;
  postfix?: string | null;
}
/**
 * Describes the constraints of a float input.
 *
 * Used to generate html markup for changing fields of config structs.
 */

export interface FloatInput {
  /**
   * The minimum value this input can be
   */
  min?: number | null;
  /**
   * The maximum value this input can be
   */
  max?: number | null;
  /**
   * The amount this input will step up and down
   */
  step?: number | null;
  postfix?: string | null;
  scale: number;
}

export interface StrClass {
  regex?: string | null;
}

export interface EnumClass {
  variants: string[];
}

export interface BoolClass {}

export interface CategoryClass {}

export interface CmdClass {
  completions: boolean;
}

export interface CompletionInfo {
  arg_descriptors: ArgDescriptor[];
  cursor_word_index: number;
}

export interface ArgDescriptor {
  value: string;
  name: string;
  description: string;
  completionKey: CompletionKey;
}
/**
 * A completion.
 *
 * Completions are generated from gpu-common types as part of the bindgen, but also at runtime for completions that cannot be known statically (i.e. file paths, resources).
 */

export interface CompletionEntry {
  /**
   * A list of aliases this completion will match on. It will always insert the first alias in the vector.
   */
  matches: string[];
  /**
   * The snippet information on the right side of a completion.
   */
  snippetText: string;
  /**
   * A long description of this completion.
   */
  description: string;
}
