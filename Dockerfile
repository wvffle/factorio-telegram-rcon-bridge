FROM rust:latest AS builder
WORKDIR /usr/src/cracktorio-bot
COPY Cargo.toml .
COPY Cargo.lock .
COPY src ./src
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && \
    apt-get install -y libssl3 ca-certificates && \
    rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/cracktorio-bot/target/release/cracktorio-bot /usr/local/bin/cracktorio-bot
CMD ["cracktorio-bot"]
