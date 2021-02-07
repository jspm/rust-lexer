.DEFAULT_GOAL := build-web

test:
	cargo test

bench:
	cargo bench

build-web:
	wasm-pack build --release --out-dir wasm_web --target web -- --features wasm
	@rm -f wasm_web/README.md wasm_web/.gitignore
	@echo "\033[0;32mwasm file gzipped size:\033[0m"
	@gzip -c wasm_web/es_module_lexer_bg.wasm | wc -c

build-node:
	wasm-pack build --release --out-dir wasm_node --target nodejs -- --features wasm

wasm-bench: build-node
	node --expose-gc wasm_benches/benchmark.mjs

wasm-web-test: build-web
	@echo "\033[0;32mTo run the tests open http://localhost:8080/wasm_tests/ in your browser\033[0m"
	@npx wmr --public .

.PHONY: test bench build-web build-node wasm-bench wasm-web-test
