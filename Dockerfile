FROM rust:1.68 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:buster-slim
WORKDIR /app
COPY --from=builder /app/target/release/windexer .
COPY config /app/config
CMD ["./windexer"]