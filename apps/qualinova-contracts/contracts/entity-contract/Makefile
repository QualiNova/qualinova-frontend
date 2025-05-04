default: build

all: test

test: build
	cargo test

build:
	stellar contract build
	@ls -l /Users/godswill/web3projects/Qualinova/qualinova-dev/apps/qualinova-contracts/target/wasm32-unknown-unknown/release/*.wasm

fmt:
	cargo fmt --all

clean:
	cargo clean