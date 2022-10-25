FROM rust:1.64 AS builder

WORKDIR /src
COPY ./frontend ./frontend
COPY ./backend ./backend
COPY ./admin-dash ./admin-dash
COPY ./common ./common
RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk
RUN cargo install wasm-bindgen-cli
RUN cd ./frontend && trunk build --release
RUN cd ./admin-dash && trunk build --release
RUN cd ./backend && cargo build --release

FROM gcr.io/distroless/cc-debian10

WORKDIR /usr/local/bin

COPY --from=builder /src/backend/target/release/backend /usr/local/bin/backend
COPY --from=builder /src/frontend/dist /usr/local/bin/frontend/dist
COPY --from=builder /src/admin-dash/dist /usr/local/bin/admin-dash/dist

CMD [ "backend" ]
