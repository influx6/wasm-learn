deps:
	rustup add --target wasm32-unknown-unknown
	cargo install wasm-bindgen

wasm:
	cargo build --target wasm32-unknown-unknown
	mkdir -p dist
	wasm-bindgen target/wasm32-unknown-unknown/debug/bindgenhello.wasm --out-dir ./dist