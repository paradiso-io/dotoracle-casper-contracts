prepare:
	rustup target add wasm32-unknown-unknown

build-contract:
	cargo build --release -p cep78-bridge-session --target wasm32-unknown-unknown
	wasm-strip target/wasm32-unknown-unknown/release/cep78_bridge_session.wasm

clean:
	cargo clean
	rm -rf tests/wasm/*.wasm