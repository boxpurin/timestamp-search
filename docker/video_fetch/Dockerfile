FROM rust:1.90.0 AS builder

WORKDIR /app
COPY service-backend/ .

RUN cargo build --release --bin video_fetch

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/video_fetch /usr/local/bin/

CMD ["video_fetch"]