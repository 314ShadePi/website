FROM rust:1.64

WORKDIR /src/admin-dash
EXPOSE 12341
RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk
RUN cargo install wasm-bindgen-cli
