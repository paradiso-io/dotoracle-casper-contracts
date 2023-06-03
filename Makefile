prepare:
	rustup target add wasm32-unknown-unknown

build-contract:
	cd nftbridge && cargo build --release --target wasm32-unknown-unknown
	wasm-strip nftbridge/target/wasm32-unknown-unknown/release/contract.wasm 2>/dev/null | true

build-cep78-bridge-session:
	cd cep78-bridge-session && cargo build --release --target wasm32-unknown-unknown
	wasm-strip cep78-bridge-session/target/wasm32-unknown-unknown/release/cep78_bridge_session.wasm 2>/dev/null | true

build-erc20-bridge-session:
	cd erc20-bridge-session && cargo build --release --target wasm32-unknown-unknown
	wasm-strip erc20-bridge-session/target/wasm32-unknown-unknown/release/erc20_bridge_session.wasm 2>/dev/null | true


test: build-contract
	mkdir -p tests/wasm
	cp nftbridge/target/wasm32-unknown-unknown/release/contract.wasm tests/wasm
	cd tests && cargo test -- --nocapture

test-wrap-nft: build-contract
	mkdir -p tests/wasm
	cp nftbridge/target/wasm32-unknown-unknown/release/contract.wasm tests/wasm
	cd ../cep-78-enhanced-nft && make build-contract
	cp ../cep-78-enhanced-nft/contract/target/wasm32-unknown-unknown/release/contract.wasm tests/wasm/cep78wrapped.wasm
	cd tests && cargo test -- --nocapture

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
