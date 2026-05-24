FROM rust:1.88 AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

COPY . .

RUN cargo build --release --locked

FROM debian:bookworm-slim

WORKDIR /app

COPY --from=builder /app/target/release/KeyBox /app/KeyBox

CMD ["./KeyBox"]