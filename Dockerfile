# Build stage
FROM rust:1.83-slim-bullseye as builder

WORKDIR /app

# Copy the entire workspace
COPY . .

# Build the server in release mode
# This handles the workspace dependencies (poker-engine) automatically
RUN cargo build --release --bin poker-server

# Runtime stage
FROM debian:bullseye-slim
# Install OpenSSL (needed for many Rust web frameworks) and ca-certificates
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the binary from the builder stage
COPY --from=builder /app/target/release/poker-server /app/poker-server

# Copy the static UI files
# The server expects "poker-ui" in the current directory (see main.rs)
COPY --from=builder /app/poker-ui /app/poker-ui

# Set environment variables
ENV RUST_LOG=info
ENV PORT=8080

# Expose the port
EXPOSE 8080

# Run the binary
CMD ["./poker-server"]
