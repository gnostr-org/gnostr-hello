aarch64-unknown-linux-musl:## 		aarch64-unknown-linux-musl
	$(MAKE) aarch64-unknown-linux-musl
help:## 		help
	@awk 'BEGIN {FS = ":.*?##"} /^[a-zA-Z_-]+:.*?##/ {printf "\033[36m%-15s\033[0m %s\n", $$1, $$2}' $(MAKEFILE_LIST)
	@awk 'BEGIN {FS = ":.*?###"} /^[a-zA-Z_-]+:.*?###/ {printf "\033[36m%-15s\033[0m %s\n", $$1, $$2}' $(MAKEFILE_LIST)

include cargo.mk
include Makefile
cargo-run: ## 		cargo run
	cargo run --bin gnostr-hello
