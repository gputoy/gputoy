#!make
.PHONY: all clean all-clean fmt lint

# Defaults
SHELL = 		/bin/sh
DEFAULT_ENV = 	.env
target = 		dev

-include $(DEFAULT_ENV)

# Frontend constants
FRONT_DIR = front
FRONT_OUT = dist
FRONT_PACKAGE = $(FRONT_DIR)/package.json
FRONT_TYPES_GENERATOR =	$(FRONT_DIR)/generate_common_types.js
FRONT_NODE_ENV = VITE_MAKE_ANALYZER_PATH=$(target)/$(WASM_ANALYZER_MODULE_NAME).wasm \
	VITE_MAKE_CLIENT_PATH=$(target)/$(WASM_CLIENT_MODULE_NAME).wasm
_front := $(shell find $(FRONT_DIR)/src -name "*")
# Rust constants (referenced by target)
RUST_TARGET<dev> = debug
RUST_TARGET<prod> = release
RUST_OPT_FLAG<prod> = --release
RUST_LOG<dev> = debug
RUST_LOG<prod> = info
# Wasm constants
WASM_PKG = $(FRONT_DIR)/pkg
WASM_OUT = $(FRONT_DIR)/static
WASM_ANALYZER_MODULE_NAME = analyzer
WASM_CLIENT_MODULE_NAME = client
WASM_FLAGS = RUSTFLAGS=--cfg=web_sys_unstable_apis
WASM_OPT_FLAG<dev> = --dev
# Crates
GPU_COMMON = gpu-common
GPU_BACK = gpu-back
GPU_ANALYZER = gpu-analyzer
GPU_CLIENT = gpu-client
GPU_LOG = gpu-log
GPU_WASM_ANALYZER = gpu-wasm-analyzer
GPU_WASM_CLIENT = gpu-wasm-client
# Crate sources
_gpu-common := $(shell find $(GPU_COMMON) -name "*")
_gpu-back := $(shell find $(GPU_BACK) -name "*")
_gpu-analyzer := $(shell find $(GPU_ANALYZER) -name "*")
_gpu-client := $(shell find $(GPU_CLIENT) -name "*")
_gpu-log := $(shell find $(GPU_LOG) -name "*")
_gpu-wasm-analyzer := $(shell find $(GPU_WASM_ANALYZER) -name "*")
_gpu-wasm-client := $(shell find $(GPU_WASM_CLIENT) -name "*")

# -------------------------- ------- --------------------------
# -------------------------- Public --------------------------
# -------------------------- ------- --------------------------

# -------------------------- Tooling --------------------------
all:
	cargo fmt --all
	cargo clippy --all-features --workspace -- -D warnings
	cargo test --all-features --workspace

clean:
	rm -rf $(FRONT_OUT) ./target $(WASM_PKG) $(WASM_OUT)/**/*.wasm 

all-clean: clean
	rm -rf $(FRONT_DIR)/.svelte-kit $(FRONT_DIR)/node_modules schemas

fmt:
	cargo fmt --all
	npm run format --prefix $(FRONT_DIR)

lint:
	npm run lint --prefix $(FRONT_DIR)

# -------------------------- Run locally --------------------------
# Run api -- do `make api target=prod` to run in release
api: api-build
	RUST_LOG=$(RUST_LOG<$(target)>) ./target/$(RUST_TARGET<$(target)>)/gpu-back

# Run forntend -- do `make start target=prod` to run productin build
start: front-build
ifeq ($(target),dev)
	$(FRONT_NODE_ENV) npm run dev --prefix $(FRONT_DIR)
else
	$(FRONT_NODE_ENV) node ./dist/index.js
endif

# -------------------------- Nixpacks --------------------------
# Build and run nixpack image, valid options are `nix-build-front` and `nix-build-back`
nix-%: nix-build-%
	@echo Running docker image
	@echo -e '\t-env: $(DEFAULT_ENV)'
	@echo -e '\t-name: gpu-$*'
	@docker run --env-file $(DEFAULT_ENV) -it gpu-$*
	

# -------------------------- ------- --------------------------
# -------------------------- Private --------------------------
# -------------------------- ------- --------------------------


# -------------------------- Build --------------------------
# Build api executable
api-build: target/$(RUST_TARGET<$(target)>)/$(GPU_BACK)

# Exapnds from api-build
target/$(RUST_TARGET<$(target)>)/$(GPU_BACK): $(_gpu-common) $(_gpu-back)
	cargo build --package $(GPU_BACK) $(RUST_OPT_FLAG<$(target)>)

# Build wasm modules from gpu-wasm-analyzer and gpu-wasm-client
# and copy them to the static(public) frontend directory so vite
# can serve them without any name mangling
wasm-build: $(WASM_OUT)/$(target)/$(WASM_ANALYZER_MODULE_NAME).wasm $(WASM_OUT)/$(target)/$(WASM_CLIENT_MODULE_NAME).wasm

# Expands from wasm-build
.SECONDEXPANSION:
$(WASM_OUT)/$(target)/%.wasm: $(_gpu-log) $(_gpu-common) $$(_gpu-$$*) $$(_gpu-wasm-$$*)
	$(WASM_FLAGS) wasm-pack build gpu-wasm-$* --out-dir ../$(WASM_PKG)/$* --target web $(WASM_OPT_FLAG<$(target)>)
	@mkdir -p $(WASM_OUT)/$(target)
	@cp $(WASM_PKG)/$*/gpu_wasm_$*_bg.wasm $(WASM_OUT)/$(target)/$*.wasm

# Build deps and maybe build node server depending on if 
# its running in dev or prod
front-build: wasm-build $(FRONT_DIR)/node_modules types $(if $(findstring $(target),prod),$(FRONT_OUT))

# Build nodejs server for frontend
$(FRONT_OUT):
	$(FRONT_NODE_ENV) npm run build --prefix $(FRONT_DIR)

# -------------------------- Nixpacks --------------------------
# Build nixpack image, valid options are `nix-build-front` and `nix-build-back`
nix-build-%:
	@echo Building $* image 
	@echo -e '\t-config: nixpacks.$*.toml'
	@echo -e '\t-name: gpu-$*'
	@nixpacks build . -c nixpacks.$*.toml --name gpu-$*


# -------------------------- Misc --------------------------
# Make json schemas from types defined in gpu-common
schemas: $(_gpu-common) 
	cargo run --package gpu-common --features serde,schema

# Make ts types from schemas generated in `make schemas`
types: schemas
	node $(FRONT_TYPES_GENERATOR)

# Fetch npm dependencies
$(FRONT_DIR)/node_modules: $(FRONT_DIR)/package.json
	npm i --prefix front
