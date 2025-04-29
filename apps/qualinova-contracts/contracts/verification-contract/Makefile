# Makefile for the Verification Contract

# Configuration
RUSTFLAGS := -C target-cpu=native
TARGET := wasm32-unknown-unknown
CARGO_PROFILE := release

# File paths
WASM_OUTPUT := target/$(TARGET)/$(CARGO_PROFILE)/verification_contract.wasm
WASM_OPTIMIZED := target/$(TARGET)/$(CARGO_PROFILE)/verification_contract.optimized.wasm

# Build the contract
build:
	cargo build --target $(TARGET) --$(CARGO_PROFILE)

# Build with optimizations for release
build-optimized: build
	wasm-opt -Oz $(WASM_OUTPUT) -o $(WASM_OPTIMIZED)

# Run tests
test:
	cargo test

# Run tests with output
test-verbose:
	cargo test -- --nocapture

# Run specific test
test-one:
	cargo test $(TEST) -- --nocapture

# Clean build artifacts
clean:
	cargo clean
	rm -f $(WASM_OPTIMIZED)

# Install the contract (requires Soroban CLI)
install: build-optimized
	stellar contract install --wasm $(WASM_OPTIMIZED)

# Deploy the contract to testnet (requires Soroban CLI and configured account)
deploy-testnet: build-optimized
	stellar contract deploy \
		--wasm $(WASM_OPTIMIZED) \
		--network testnet \
		--source-account qualinova-admin

# Fix file fomratting
fmt:
	cargo fmt

# Generate documentation
docs:
	cargo doc --no-deps --open

.PHONY: build build-optimized test test-verbose test-one clean install deploy-testnet docs