#!make
.PHONY: all  watch-front-do watch-front

COMMON = ./gpu-common ./gpu-log
ANALYZER = ./gpu-analyzer ./gpu-wasm-analyzer
CLIENT = ./gpu-client ./gpu-wasm-client
API = ./gpu-common ./gpu-back 

WATCH_FRONT = -w gpu-common -w gpu-log -w gpu-client -w gpu-analyzer -w gpu-wasm-analyzer -w gpu-wasm-client -w front

WASM_EXP_FLAGS = RUSTFLAGS=--cfg=web_sys_unstable_apis
WASM_ARGS_ANALYZER = build ./gpu-wasm-analyzer --out-dir ../front/pkg/analyzer  --target web
WASM_ARGS_CLIENT = build ./gpu-wasm-client --out-dir ../front/pkg/client  --target web

DEFAULT_ENV = .env

-include $(DEFAULT_ENV)

# Tooling
all:
	cargo fmt --all
	cargo clippy --all-features --workspace -- -D warnings
	cargo test --all-features --workspace

clean:
	rm -rf ./dist ./target ./front/pkg ./front/static/*.wasm

all-clean: clean
	rm -rf ./front/.svelte-kit ./front/node_modules ./schemas

fmt:
	cargo fmt --all
	npm run format --prefix front

lint:
	npm run lint --prefix front


# Wasm
wasm: wasm-analyzer wasm-client cp-wasm

wasm-prod: wasm-analyzer-prod wasm-client-prod cp-wasm

cp-wasm:
	cp front/pkg/analyzer/gpu_wasm_analyzer_bg.wasm front/static/analyzer.wasm
	cp front/pkg/client/gpu_wasm_client_bg.wasm front/static/client.wasm

wasm-analyzer: $(COMMON) $(ANALYZER)
	$(WASM_EXP_FLAGS) wasm-pack $(WASM_ARGS_ANALYZER) --dev

wasm-analyzer-prod: $(COMMON) $(ANALYZER)
	$(WASM_EXP_FLAGS) wasm-pack $(WASM_ARGS_ANALYZER) 

wasm-client: $(COMMON) $(CLIENT)
	$(WASM_EXP_FLAGS) wasm-pack $(WASM_ARGS_CLIENT) --dev

wasm-client-prod: $(COMMON) $(CLIENT)
	$(WASM_EXP_FLAGS) wasm-pack $(WASM_ARGS_CLIENT)


# API
api-build: $(API)
	cargo build --package gpu-back

api: api-build
	RUST_LOG=debug ./target/debug/gpu-back

api-build-prod: $(API) 
	cargo build --package gpu-back --release

api-prod: api-build-prod
	RUST_LOG=info ./target/release/gpu-back


# Frontend
npmi:
	npm i --prefix front

front-build: wasm-prod npmi types 
	npm run build --prefix front

start: wasm npmi types
	npm run dev --prefix front -- --port ${PORT_FRONT} 

start-prod: front-build
	node ./dist/index.js

# Types
types: 
	cargo run --package gpu-common --features serde,schema
	node front/generate_common_types.js

# Nixpacks
nix-build-%:
	@echo Building $* image 
	@echo -e '\t-config: nixpacks.$*.toml'
	@echo -e '\t-name: gpu-$*'
	@nixpacks build . -c nixpacks.$*.toml --name gpu-$*

nix-%: nix-build-%
	@echo Running docker image
	@echo -e '\t-env: $(DEFAULT_ENV)'
	@echo -e '\t-name: gpu-$*'
	@docker run --env-file $(DEFAULT_ENV) -it gpu-$*

# Watch
watch-front:
	cargo watch $(WATCH_FRONT) -- make watch-front-do

watch-front-do:
	$(WASM_EXP_FLAGS) wasm-pack $(WASM_ARGS_ANALYZER) --dev
	$(WASM_EXP_FLAGS) wasm-pack $(WASM_ARGS_CLIENT) --dev
	npm run dev --prefix front -- --port ${PORT_FRONT}
