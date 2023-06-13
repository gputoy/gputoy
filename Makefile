#!make
.PHONY: all clean all-clean fmt lint bindgen

# Defaults
SHELL = /bin/sh
DEFAULT_ENV = .env
# Target to build for, options = [dev, prod]
# To ovveride, appene target=prod at end of make command
target = dev
TEMPLATES = templates

-include $(DEFAULT_ENV)

# Frontend constants

# Frontend source code
FRONT_DIR = front
# Frontend build output
FRONT_OUT = dist
FRONT_PACKAGE = $(FRONT_DIR)/package.json
# Environment variables to export when running the frontend server
FRONT_NODE_ENV = VITE_MAKE_ANALYZER_PATH=$(target)/$(WASM_ANALYZER_MODULE_NAME).wasm \
	VITE_MAKE_CLIENT_PATH=$(target)/$(WASM_CLIENT_MODULE_NAME).wasm \
	VITE_MAKE_COMMON_PATH=$(target)/$(WASM_COMMON_MODULE_NAME)
# All frontend source files (for dependencies)
_front := $(shell find ${FRONT_DIR} -not \( -path front/src/routes -prune \) -not \( -path front/node_modules -prune \) -not \( -path front/.svelte-kit -prune \) -name "*")

# Rust constants (referenced by target)

# Target when running dev
RUST_TARGET<dev> = debug
# Target when running prod
RUST_TARGET<prod> = release
# Flags to pass to cargo build for prod
RUST_OPT_FLAG<prod> = --release
# Log level for dev
RUST_LOG<dev> = debug
# Log level for prod
RUST_LOG<prod> = info

# Bindgen config path
BINDGEN_CONFIG_PATH = $(GPU_BINDGEN)/config.toml
export BINDGEN_CONFIG_PATH
# Where generated code and json are output to
BINDGEN_OUT = $(FRONT_DIR)/generated
export BINDGEN_OUT
# Path to json -> ts generator script
BINDGEN_JSON_TO_TS_SCRIPT = $(FRONT_DIR)/json-to-ts.js
export BINDGEN_JSON_TO_TS_SCRIPT
# Bindgen config template path 
BINDGEN_CONFIG_TMPL = $(TEMPLATES)/config.bindgen.toml.tmpl

# Wasm constants

# Where built wasm modules are output to
WASM_OUT = $(FRONT_DIR)/static
# Name of common wasm module (this one needs the .wasm or else it wont work)
WASM_COMMON_MODULE_NAME = common.wasm
# Name of analyzer wasm module
WASM_ANALYZER_MODULE_NAME = analyzer
# Name of client wasm module
WASM_CLIENT_MODULE_NAME = client
# Flags to pass to wasm-pack
WASM_FLAGS = RUSTFLAGS=--cfg=web_sys_unstable_apis
# Opt flags to pass to wasm-pack in dev
WASM_OPT_FLAG<dev> = --dev

THEME_JSON_OUT = $(BINDGEN_OUT)/themes.json
CSS_TO_JSON_SCRIPT = $(FRONT_DIR)/css-to-json.js
_css-themes := $(shell find $(FRONT_DIR)/src/styles/themes -name "*")

# Crates

GPU_COMMON = gpu-common
GPU_BACK = gpu-back
GPU_BINDGEN = gpu-bindgen
GPU_ANALYZER = gpu-analyzer
GPU_CLIENT = gpu-client
GPU_LOG = gpu-log
GPU_WASM_COMMON = gpu-wasm-common
GPU_WASM_ANALYZER = gpu-wasm-analyzer
GPU_WASM_CLIENT = gpu-wasm-client

# Crate sources

_gpu-common := $(shell find $(GPU_COMMON) -name "*")
_gpu-back := $(shell find $(GPU_BACK) -name "*")
_gpu-bindgen := $(shell find $(GPU_BINDGEN) -name "*")
_gpu-analyzer := $(shell find $(GPU_ANALYZER) -name "*")
_gpu-client := $(shell find $(GPU_CLIENT) -name "*")
_gpu-log := $(shell find $(GPU_LOG) -name "*")
_gpu-wasm-common := $(shell find $(GPU_WASM_COMMON) -name "*")
_gpu-wasm-analyzer := $(shell find $(GPU_WASM_ANALYZER) -name "*")
_gpu-wasm-client := $(shell find $(GPU_WASM_CLIENT) -name "*")

# -------------------------- ------- --------------------------
# -------------------------- Public --------------------------
# -------------------------- ------- --------------------------

# -------------------------- Tooling --------------------------
all: clippy fmt test

clippy:
	cargo clippy --all-features --workspace -- -D warnings

clean:
	cargo clean
	rm -rf $(FRONT_OUT) ./target $(BINDGEN_OUT)/* $(WASM_OUT)/**/*.wasm 

all-clean: clean
	rm -rf $(FRONT_DIR)/.svelte-kit $(FRONT_DIR)/node_modules

fmt:
	cargo fmt --all
	npm run format --prefix $(FRONT_DIR)

test:
	cargo test --all-features --workspace --exclude proc-macro

lint:
	npm run lint --prefix $(FRONT_DIR)

bindgen:
	cargo run --package $(GPU_BINDGEN)

# -------------------------- Run locally --------------------------
# Run api -- do `make api target=prod` to run in release
api: api-build
	RUST_LOG=$(RUST_LOG<$(target)>) ./target/$(RUST_TARGET<$(target)>)/gpu-back

# Run frontend -- do `make start target=prod` to run productin build
start: front-build
ifeq ($(target),dev)
	@echo "Starting svelte server in dev mode"
	@$(FRONT_NODE_ENV) npm run dev --prefix $(FRONT_DIR)
else
	@echo "Starting svelte server in production mode"
	$(FRONT_NODE_ENV) node ./dist/index.js
endif

# -------------------------- Nixpacks --------------------------
# Build and run nixpack image, valid options are `nix-front` and `nix-back`
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
wasm-build: $(WASM_OUT)/$(target)/$(WASM_ANALYZER_MODULE_NAME).wasm \
	$(WASM_OUT)/$(target)/$(WASM_CLIENT_MODULE_NAME).wasm \
	$(WASM_OUT)/$(target)/$(WASM_COMMON_MODULE_NAME)

# Expands from wasm-build
.SECONDEXPANSION:
$(WASM_OUT)/$(target)/%.wasm: $(_gpu-log) $(_gpu-common) $$(_gpu-$$*) $$(_gpu-wasm-$$*)
	$(WASM_FLAGS) wasm-pack build gpu-wasm-$* \
		--out-dir ../$(BINDGEN_OUT)/$* \
		--target web $(WASM_OPT_FLAG<$(target)>)
	@mkdir -p $(WASM_OUT)/$(target)
	@cp $(BINDGEN_OUT)/$*/gpu_wasm_$*_bg.wasm $(WASM_OUT)/$(target)/$*.wasm

# Build deps and maybe build node server depending on if 
# its running in dev or prod
front-build: $(FRONT_DIR)/node_modules $(BINDGEN_OUT)/common.ts wasm-build $(THEME_JSON_OUT) $(if $(findstring $(target),prod),$(FRONT_OUT))

# Build nodejs server for frontend
$(FRONT_OUT): $(_front)
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
$(BINDGEN_OUT)/common.ts: $(_gpu-common) $(_gpu_bindgen) $(BINDGEN_CONFIG_PATH)
	@RUST_LOG=INFO cargo run --package $(GPU_BINDGEN) 

$(BINDGEN_CONFIG_PATH): $(BINDGEN_CONFIG_TMPL)
	cat $(BINDGEN_CONFIG_TMPL) | envsubst > $(BINDGEN_CONFIG_PATH)

# Fetch npm dependencies
$(FRONT_DIR)/node_modules: $(FRONT_DIR)/package.json
	npm i --prefix $(FRONT_DIR)

$(THEME_JSON_OUT): $(_css-themes)
	@node $(CSS_TO_JSON_SCRIPT) $(THEME_JSON_OUT)
