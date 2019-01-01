run:
	cargo run

test:
	rustc --version && cargo --version
	RUST_BACKTRACE=1 cargo test --all --verbose -- --test-threads=16 --nocapture

update:
	cargo update

.PHONY: run test update
