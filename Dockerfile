FROM rust:latest AS builder
WORKDIR /usr/src/cracktorio-bot
COPY . .
RUN cargo build --release

FROM debian:buster-slim
COPY --from=builder /usr/src/cracktorio-bot/target/release/cracktorio-bot /usr/local/bin/cracktorio-bot
CMD ["cracktorio-bot"]
