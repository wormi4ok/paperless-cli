build:
	cargo build --release

lint:
	cargo fmt
	cargo clippy
