run:
	cargo run

test:
	cargo test --all -- --nocapture

update:
	cargo update

.PHONY: run test update
