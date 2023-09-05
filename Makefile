fix:
	cargo clippy --fix --allow-dirty --allow-staged -- -D warnings

build:
	cargo build --release
