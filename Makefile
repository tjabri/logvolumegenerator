.phony: test
test:
	cargo test --verbose

.phony: check
check:
	cargo check

.phony: build-release
build-release:
	cargo build --release
