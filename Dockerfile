FROM rust:1.64

WORKDIR /src
EXPOSE 8080
RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk
RUN cargo install wasm-bindgen-cli
