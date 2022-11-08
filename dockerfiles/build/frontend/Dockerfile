FROM rust:latest

WORKDIR /src/frontend
RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk
RUN cargo install wasm-bindgen-cli
