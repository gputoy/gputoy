/* tslint:disable */
/**
 * This file was automatically generated by json-schema-to-typescript.
 * DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
 * and run json-schema-to-typescript to regenerate this file.
 */



export type Action =
  | {
      togglePane: Pane;
    }
  | {
      /**
       * @minItems 2
       * @maxItems 2
       */
      shiftPane: [Pane, number];
    }
  | "playPause"
  | "reset"
  | "rebuild";

export type Pane = "editorPane" | "projectPane" | "resourcePane";

export type PerformanceLevel = "Default" | "PowerSaver";

export interface Config {
  limitFps?: number;
  perfLevel?: PerformanceLevel | null;
}

export interface Credentials {
  password: string;
  username_or_email: string;
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
  layout?: null;
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

export interface Layout {
  /**
   * Currently opened file index within workspace
   */
  fileIndex?: number | null;
  /**
   * Is the left side status panel open
   */
  isStatusOpen: boolean;
  /**
   * List of file identifiers which is open in workspace. Order of identifiers in vec is the order it is listed in the editor.
   */
  workspace: string[];
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

export interface UserConfig {
  keybinds: {
    [k: string]: Action;
  };
}