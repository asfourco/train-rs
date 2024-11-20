format:
	cargo fmt

lint:
	cargo clippy

build: lint format
	cargo build --release

run: build
	./target/release/train-rs