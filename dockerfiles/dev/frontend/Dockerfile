FROM rust:latest

WORKDIR /src/frontend
EXPOSE 8080
RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk
RUN cargo install wasm-bindgen-cli
