GNOSTR_HELLO_SRC := $(shell find ./src) Cargo.toml Cargo.lock

aarch64-unknown-linux-musl:gnostr-hello/target/aarch64-unknown-linux-musl/release/gnostr-hello
gnostr-hello/target/aarch64-unknown-linux-musl/release/gnostr-hello: $(GNOSTR_HELLO_SRC)
	docker run --rm -it -v ~/.cargo/registry:/root/.cargo/registry -v "$(shell pwd)":/home/rust/src messense/rust-musl-cross:aarch64-musl cargo build --release
