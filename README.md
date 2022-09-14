# **Gputoy**

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
cargo dev
```
Build wasm module
```console
cargo build-wasm
```
Run backend
```console
cargo api
```

## Build image
 ```console
  cargo install nixpacks
 ```

```
  nixpacks build .
````

Once build finished, it should output:
```console
=== Successfully Built! ===                                                             

Run:
  docker run -it a4030551-7f53-4df1-aec5-cf91a8e954e4

```
Then run the newly generated image on docker, exposing the default port:
```console
  docker run -it -p 3000:3000 a4030551-7f53-4df1-aec5-cf91a8e954e4
```




