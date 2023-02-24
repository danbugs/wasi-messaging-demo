.PHONY: build
build:
	@echo "Building..."
	@cargo build --manifest-path ./guest/Cargo.toml --target wasm32-wasi

.PHONY: componentize
componentize:
	@echo "Making component..."
	@wasm-tools component new ./target/wasm32-wasi/debug/guest.wasm -o guest.component.wasm --adapt ./wasi_snapshot_preview1.wasm

.PHONY: run
run:
	@echo "Running..."
	@cargo run --manifest-path ./host/Cargo.toml