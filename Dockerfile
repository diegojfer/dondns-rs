FROM rust:1-slim-bookworm AS builder

RUN apt-get update && apt-get install -y libssl-dev pkg-config

WORKDIR /app

COPY src ./src
COPY Cargo.toml ./

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y libssl3 ca-certificates

COPY --from=builder /app/target/release/dondns-rs /usr/local/bin/dondns-rs

ENTRYPOINT ["/usr/local/bin/dondns-rs"]
