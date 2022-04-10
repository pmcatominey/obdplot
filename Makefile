.PHONY: check
check:
	cargo check --workspace --all-targets
	cargo fmt --all -- --check
	cargo clippy --workspace --all-targets --all-features --  -D warnings -W clippy::all

.PHONY: run
run:
	cargo run --release

.PHONY: install
install:
	cargo install --path .