CARGO_NEXTEST := cargo nextest

# Clippy
.PHONY: clippy
clippy:
	 cargo clippy --all-features -- -D warnings -D clippy::all -D clippy::nursery -D clippy::integer_division -D clippy::arithmetic_side_effects -D clippy::style -D clippy::perf

# Test
.PHONY: test
test: 
	cargo-build-sbf
	SBF_OUT_DIR=$(pwd)/target/sbf-solana-solana/release
	$(CARGO_NEXTEST) run --all-features
