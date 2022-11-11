/* tslint:disable */
/**
 * This file was automatically generated by json-schema-to-typescript.
 * DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
 * and run json-schema-to-typescript to regenerate this file.
 */



export type Action =
  | {
      c: Panel;
      ty: "togglePanel";
    }
  | {
      ty: "toggleDebugPanel";
    }
  | {
      ty: "toggleUserPreferences";
    }
  | {
      c: ShiftPaneArgs;
      ty: "shiftPanel";
    }
  | {
      ty: "playPause";
    }
  | {
      ty: "reset";
    }
  | {
      ty: "rebuild";
    }
  | {
      ty: "toggleConsole";
    }
  | {
      c: Panel;
      ty: "focus";
    }
  | {
      ty: "closeDocument";
    }
  | {
      ty: "nextDocument";
    }
  | {
      ty: "previousDocument";
    }
  | {
      ty: "createNewProject";
    }
  | {
      ty: "createNewFile";
    }
  | {
      ty: "saveProjectToRemote";
    }
  | {
      ty: "saveCurrentFile";
    }
  | {
      ty: "saveAllFiles";
    }
  | {
      ty: "fork";
    }
  | {
      ty: "publish";
    }
  | {
      ty: "closeFile";
    }
  | {
      ty: "closeProject";
    };

export type Panel = "editorPanel" | "projectPanel" | "resourcePanel";

export interface ShiftPaneArgs {
  pane: Panel;
  shift: number;
}

export type PerformanceLevel = "Default" | "PowerSaver";

export interface Config {
  limitFps?: number;
  perfLevel?: PerformanceLevel | null;
}

export interface Credentials {
  password: string;
  usernameOrEmail: string;
}

export interface LoginResponse {
  userId: string;
}

export interface NewUser {
  email: string;
  password: string;
  username: string;
}

export interface NewUserResponse {
  id: string;
}

export type SupportedExtension = "wgsl" | "glsl" | "txt" | "md" | "json" | "csv" | "png" | "jpeg" | "mp3";

export interface Project {
  config?: Config | null;
  files: Files;
  layout?: Layout | null;
}

export interface Config {
  limitFps?: number;
  perfLevel?: PerformanceLevel | null;
}
/**
 * Gputoy virtual directory. Each file in the map has its path from root as key, including file name and extension
 *
 * example: ```ts map: { "shaders/main.wgsl": { "data": "...", "dir": "shaders/", "fileName": "main", "extension": "wgsl", } } ```
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
   * File extension
   */
  extension: SupportedExtension;
  /**
   * Fetch url. If exists, then contents will be fetched from remote URL on project load
   */
  fetch?: string | null;
  /**
   * Name of file
   */
  fileName: string;
}

export interface Layout {
  /**
   * Panel settings for editorPanel
   */
  editorPanel: PanelState;
  /**
   * Currently opened file index within workspace
   */
  fileIndex?: number | null;
  /**
   * Is the left side status panel open
   */
  isStatusOpen: boolean;
  /**
   * Panel settings for projectPanel
   */
  projectPanel: PanelState;
  /**
   * Panel settings for resourcePanel
   */
  resourcePanel: PanelState;
  /**
   * List of file identifiers which is open in workspace. Order of identifiers in vec is the order it is listed in the editor.
   */
  workspace: string[];
}

export interface PanelState {
  show: boolean;
  size: number;
}

export interface ProjectResponse {
  authorId?: string | null;
  config?: Config | null;
  createdAt: string;
  description?: string | null;
  files: Files;
  forkedFromId?: string | null;
  id: string;
  layout?: Layout | null;
  published: boolean;
  title: string;
  updatedAt: string;
}

export interface ProjectUpsert {
  config?: Config | null;
  description?: string | null;
  files: Files;
  id?: string | null;
  layout?: Layout | null;
  published: boolean;
  title: string;
}

export type LineNumberCOnfig = "on" | "interval" | "relative" | "off";

export interface UpdateUserInfoArgs {
  bio?: string | null;
  config?: UserConfig | null;
  fullName?: string | null;
  image?: string | null;
}

export interface UserConfig {
  editor: UserEditorConfig;
  general: UserGeneralConfig;
  keybinds: {
    [k: string]: FilteredAction;
  };
  theme: {
    [k: string]: string;
  };
}

export interface UserEditorConfig {
  fontFamily?: string | null;
  fontSize?: number | null;
  lineNumbers: LineNumberCOnfig;
}

export interface UserGeneralConfig {
  editorPanelSize: number;
  projectPanelSize: number;
  resourcePanelSize: number;
}

export interface FilteredAction {
  action: Action;
  condition?: string | null;
}

export interface UserInfoResponse {
  active: boolean;
  bio?: string | null;
  config?: UserConfig | null;
  createdAt: string;
  email: string;
  emailVerified: boolean;
  fullName?: string | null;
  id: string;
  image?: string | null;
  updatedAt: string;
  username: string;
}