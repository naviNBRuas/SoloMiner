# Use a Rust base image
FROM rust:latest as builder

# Set the working directory
WORKDIR /app

# Copy Cargo.toml and Cargo.lock to leverage Docker cache
COPY Cargo.toml Cargo.lock ./

# Copy the source code
COPY src ./src

# Build the release binary
RUN cargo build --release

# Use a minimal base image for the final stage
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libssl-dev && rm -rf /var/lib/apt/lists/*

# Set the working directory
WORKDIR /app

# Copy the built binary from the builder stage
COPY --from=builder /app/target/release/solominer .

# Copy the config.toml
COPY config.toml .

# Expose the dashboard port
EXPOSE 8080

# Set the entrypoint to run the miner
ENTRYPOINT ["./solominer"]
