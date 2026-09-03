FROM rust:1.90 AS builder

WORKDIR /app

COPY Cargo.toml ./
COPY src ./src

RUN cargo build --release

FROM debian:trixie-slim

WORKDIR /app

COPY --from=builder /app/target/release/middleware ./middleware

CMD ["./middleware"]
