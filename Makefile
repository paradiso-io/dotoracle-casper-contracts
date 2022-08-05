prepare:
	rustup target add wasm32-unknown-unknown

build-contract:
	cd nftbridge && cargo build --release --target wasm32-unknown-unknown
	wasm-strip nftbridge/target/wasm32-unknown-unknown/release/contract.wasm 2>/dev/null | true

test: build-contract
	mkdir -p tests/wasm
	cp nftbridge/target/wasm32-unknown-unknown/release/contract.wasm tests/wasm
	cd tests && cargo test

clippy:
	cd nftbridge && cargo clippy --all-targets -- -D warnings
	cd tests && cargo clippy --all-targets -- -D warnings

check-lint: clippy
	cd nftbridge && cargo fmt -- --check
	cd tests && cargo fmt -- --check

lint: clippy
	cd nftbridge && cargo fmt
	cd tests && cargo fmt

clean:
	cd nftbridge && cargo clean
	cd tests && cargo clean
	rm -rf tests/wasm
