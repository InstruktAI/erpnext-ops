.PHONY: install format lint test build

install:
	cargo fetch

format:
	cargo fmt

lint:
	cargo clippy --all-targets -- -D warnings

test:
	cargo test

build:
	cargo build --release
