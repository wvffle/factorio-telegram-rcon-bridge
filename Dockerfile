FROM rust:latest AS builder
WORKDIR /usr/src/cracktorio-bot
COPY . .
RUN cargo build --release

FROM debian:buster-slim
RUN apt-get update && \
    apt-get install -y libssl-dev ca-certificates && \
    rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/cracktorio-bot/target/release/cracktorio-bot /usr/local/bin/cracktorio-bot
CMD ["cracktorio-bot"]
