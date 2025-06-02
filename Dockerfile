# Dockerfile for xelarius-node
FROM rust:1.76.0 as builder
WORKDIR /app
COPY . .
RUN cargo build --release --locked -p xelarius-node

FROM debian:buster-slim
WORKDIR /app
COPY --from=builder /app/target/release/xelarius-node ./xelarius-node
CMD ["./xelarius-node"]
