format:
	cargo fmt

lint:
	cargo clippy

build:
	cargo build --release

run: build
	./target/release/train-rs

test:
	cargo test