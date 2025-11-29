FROM rust:1.90.0 AS builder

WORKDIR /app
COPY service-backend/ .

RUN cargo build --release --bin restful_server

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/restful_server /usr/local/bin/
EXPOSE 8080
CMD ["restful_server"]