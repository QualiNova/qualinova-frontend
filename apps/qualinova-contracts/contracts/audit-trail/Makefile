all: build

build:
	cargo build --target wasm32-unknown-unknown --release
	soroban contract optimize --wasm ../../target/wasm32-unknown-unknown/release/qualinova_audit_trail.wasm

build-optimized:
	cargo build --target wasm32-unknown-unknown --release
	soroban contract optimize --wasm ../../target/wasm32-unknown-unknown/release/qualinova_audit_trail.wasm

test:
	cargo test

clean:
	cargo clean

.PHONY: all build build-optimized test clean