# Build stage
FROM rust:1.75-slim as builder

WORKDIR /app
COPY . .
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/Rust_CLI_Tool /app/Rust-CLI-Tool

EXPOSE 8080

CMD ["/app/Rust-CLI-Tool"]
