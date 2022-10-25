FROM rust:1.64

WORKDIR /src/frontend
RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk
RUN cargo install wasm-bindgen-cli
