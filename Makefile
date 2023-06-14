PINNED_TOOLCHAIN := $(shell cat rust-toolchain)
prepare:
	rustup target add wasm32-unknown-unknown
	rustup component add clippy --toolchain ${PINNED_TOOLCHAIN}
	rustup component add rustfmt --toolchain ${PINNED_TOOLCHAIN}

build-contract:
	cd nftbridge && cargo build --release --target wasm32-unknown-unknown
	wasm-strip nftbridge/target/wasm32-unknown-unknown/release/contract.wasm 2>/dev/null | true

build-cep78-bridge-session:
	cd cep78-bridge-session && cargo build --release --target wasm32-unknown-unknown
	wasm-strip cep78-bridge-session/target/wasm32-unknown-unknown/release/cep78_bridge_session.wasm 2>/dev/null | true

build-erc20-bridge-session:
	cd erc20-bridge-session && cargo build --release --target wasm32-unknown-unknown
	wasm-strip erc20-bridge-session/target/wasm32-unknown-unknown/release/erc20_bridge_session.wasm 2>/dev/null | true

build-contracts: build-contract build-cep78-bridge-session build-erc20-bridge-session
	mkdir -p target
	cp nftbridge/target/wasm32-unknown-unknown/release/contract.wasm target/nftbridge.wasm
	cp cep78-bridge-session/target/wasm32-unknown-unknown/release/cep78_bridge_session.wasm target/
	cp erc20-bridge-session/target/wasm32-unknown-unknown/release/erc20_bridge_session.wasm target/

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
	cd erc20-bridge-session && cargo fmt -- --check
	cd cep78-bridge-session && cargo fmt -- --check
	cd tests && cargo fmt -- --check

lint: clippy
	cd nftbridge && cargo fmt
	cd erc20-bridge-session && cargo fmt
	cd cep78-bridge-session && cargo fmt
	cd tests && cargo fmt

clean:
	cd nftbridge && cargo clean
	cd erc20-bridge-session && cargo clean
	cd cep78-bridge-sessio && cargo clean
	cd tests && cargo clean
	rm -rf tests/wasm
