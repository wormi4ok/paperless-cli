build:
	cargo build --release

lint:
	cargo fmt
	cargo clippy

test:
	cargo test
