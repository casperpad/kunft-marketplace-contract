ALL_CONTRACTS = kunftmarketplace-contract
CONTRACT_TARGET_DIR = target/wasm32-unknown-unknown/release
prepare:
	rustup target add wasm32-unknown-unknown

build-contracts:
	cargo build --release --target wasm32-unknown-unknown $(patsubst %, -p %, $(ALL_CONTRACTS))
	$(foreach WASM, $(ALL_CONTRACTS), wasm-strip $(CONTRACT_TARGET_DIR)/$(subst -,_,$(WASM)).wasm 2>/dev/null | true;)

test: build-contracts
	cd tests && cargo test

clippy:
	cargo clippy --all-targets -- -D warnings
	cd tests && cargo clippy --all-targets -- -D warnings

check-lint: clippy
	cargo fmt -- --check
	cd tests && cargo fmt -- --check

lint: clippy
	cargo fmt
	cd tests && cargo fmt

clean:
	cd contract && cargo clean
	cd tests && cargo clean
	rm -rf tests/wasm
