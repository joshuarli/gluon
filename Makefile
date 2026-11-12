.PHONY: lint clean

lint:
	cargo clippy --workspace --all-targets -- -D warnings

clean:
	cargo clean
