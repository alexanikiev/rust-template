# --- Stage 1: Build the Rust application ---
FROM rust:1.74-alpine AS builder

# Install required dependencies for Alpine
RUN apk add --no-cache musl-dev alpine-sdk

WORKDIR /app
COPY . .

# Ensure dependencies are resolved separately for better caching
RUN cargo fetch

# Build the Rust application in release mode
RUN cargo build --release

# --- Stage 2: Run the application ---
FROM alpine:latest

WORKDIR /app

# Copy only the compiled binary
COPY --from=builder /app/target/release/solver /app/solver

# Set execute permissions
RUN chmod +x /app/solver

# Run the application
CMD ["/app/solver"]
