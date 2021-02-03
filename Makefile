.DEFAULT_GOAL := build-web

build-web:
	wasm-pack build --release --out-dir wasm_web --target web -- --features wasm
	rm -f wasm_web/README.md wasm_web/.gitignore

build-node:
	wasm-pack build --release --out-dir wasm_node --target nodejs -- --features wasm

wasm-size: build
	gzip -c wasm_web/rust_lexer_bg.wasm | wc -c

wasm-bench: build-node
	node --expose-gc wasm_benches/benchmark.mjs

.PHONY: build-web build-node wasm-size wasm-bench
