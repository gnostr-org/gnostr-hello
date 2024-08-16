FROM rust:latest as base
RUN apt-get update -y || true
RUN apt-get install tini -y || true
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
COPY . .
RUN cargo install --path .
FROM base as gnostr-hello
EXPOSE 8080 80 81
ENTRYPOINT tini gnostr-hello
