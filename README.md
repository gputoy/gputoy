
## **Note**: Until webgpu is stabilized across all browsers, `gputoy` will only run on browsers in [this](https://caniuse.com/webgpu) list.
# **gputoy**

[gputoy](https://gputoy.io) is a gpu shader development enviroment designed to be as simple as possible to create and share creations. 


## MVP:

 * **Frictionless**
    * Instantly create a new project, regardless if user has account or is logged in (fallback to local storage). 
    * One click sharing, even when not logged in (temporary project will be held in database).
    * Keybinds for every action, should be able to do almost everything mouseless.
  * **Flexibile**
    * Allow users to make all sorts of things like visual art, massively parallel simulations, music visualizers, live performances, and more.
    * Support wgsl and glsl shader file formats.
    * Highly configurable development environement.
  * **Featureful**
    * Interactive control over gpu resources in a variety of ways, i.e. pedal-board-like interface where user can lay out shader parameters on a grid for live tuning. 
    * I/O for gpu resources (i.e. can easily import image or soundfile and use it as a buffer). 
    * Code editor with full syntax highlighting and intellisense for shaders.

## Crates

  * [gpu-common](gpu-common) - Crate encapsulating types common between frontend client and backend server. `Json` schemas and `typescript` types are generated from this crate.
  * [gpu-back](gpu-back) - Backend crate that handles data storage, authentication, and api endpoints. Full list of endpoints can be found in the crate `README`.
  * [gpu-client](gpu-client) - Platform agnostic gputoy client library that is used in `gpu-wasm` and `gpu-tauri`. Handles building projects, rendering, and gpu resource IO.
  * [gpu-analyzer](gpu-analyzer) - Generates runnable for `gpu-client` by pre-processing and compiling project files and configs.
  * [gpu-wasm](gpu-wasm) - Wasm client for running web application at [gputoy.io/dev](https://gputoy.io/dev).
  * [gpu-tauri](gpu-tauri) - Tauri client for running native application. **(TODO)**

## Dependencies

  * Rust
  * Cargo
  * wasm-pack 
  * nodejs
  * openssl
  * Docker

## Build locally

```console
cargo install cargo-make
```
Build wasm module and start svelte-kit server with:
```console
make start
make start-prod
```
Build wasm module
```console
cargo wasm
cargo wasm-prod
```
Run backend
```console
cargo api
cargo api-prod
```

Rebuild json schemas and typescript types from `gpu-common` types
```console
cargo types
```

## Build image

Install nixpacks
 ```console
cargo install nixpacks
 ```

Build frontend image which will serve `sveltekit` server from port 3000
```console
nixpacks build -c nixpacks.front.toml .
````

Build backend image which will serve `gpu-back`
```console
nixpacks build -c nixpacks.back.toml .
```




