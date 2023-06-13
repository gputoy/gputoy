
## **Note**: Until webgpu is stabilized across all browsers, `gputoy` will only run on browsers in [this](https://caniuse.com/webgpu) list.

# **gputoy**

[gputoy](https://gputoy.io) is a platform for creating and sharing realtime art.

## Getting started

### Dependencies

  * Rust
  * Cargo
  * wasm-pack 
  * nodejs
  * openssl
  * Docker

### Build locally

Check, format, test
```console
make all
```
Build frontend dependencies and start node server
```console
make start
make start target=prod
```
Start api server
```console
make api
make api target=prod
```

Additionally, `mprocs` can be used to streamline development
```console
cargo install mprocs
mprocs
```

### Build image

Install nixpacks
 ```console
cargo install nixpacks
 ```

Build frontend image which will serve `sveltekit` server from port 3000
```console
make nix-front
````

Build backend image which will serve `gpu-back`
```console
make nix-back
```