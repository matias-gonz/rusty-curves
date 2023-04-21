test:
	cargo test

clippy:
	cargo clippy --all --all-targets -- -D warnings
