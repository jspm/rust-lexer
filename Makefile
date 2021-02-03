.DEFAULT_GOAL := build-web

test:
	cargo test

bench:
	cargo bench

build-web:
	wasm-pack build --release --out-dir wasm_web --target web -- --features wasm
	rm -f wasm_web/README.md wasm_web/.gitignore

build-node:
	wasm-pack build --release --out-dir wasm_node --target nodejs -- --features wasm

wasm-size: build-web
	gzip -c wasm_web/es_module_lexer_bg.wasm | wc -c

wasm-bench: build-node
	node --expose-gc wasm_benches/benchmark.mjs

.PHONY: test bench build-web build-node wasm-size wasm-bench
