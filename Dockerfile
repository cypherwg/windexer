FROM rust:1.75 as builder
WORKDIR /usr/src/windexer
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y libssl1.1 && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/windexer/target/release/windexer /usr/local/bin/windexer
CMD ["windexer"]