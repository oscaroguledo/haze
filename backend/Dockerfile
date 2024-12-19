# Use the Rust official image as a base image for the build stage
FROM rust:1.72 as builder

# Set the working directory in the container
WORKDIR /usr/src/app

# Copy the Cargo.toml and Cargo.lock files first to leverage Docker caching
COPY Cargo.toml Cargo.lock ./

# Build dependencies to take advantage of Docker cache
RUN cargo build --release

# Copy the source code
COPY src ./src

# Build the project in release mode
RUN cargo build --release

# Use the regular version of Ubuntu for a lighter base image
FROM ubuntu:22.04

# Install necessary runtime dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    libc6 \
    libpq-dev \
    curl \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy the compiled Rust binary from the builder image
COPY --from=builder /usr/src/app/target/release/backend /usr/local/bin/backend

# Copy the entrypoint script into the container
COPY entrypoint.sh /usr/local/bin/entrypoint.sh

# Give executable permissions to the entrypoint script
RUN chmod +x /usr/local/bin/entrypoint.sh

# Set the entrypoint script to be executed when the container starts
ENTRYPOINT ["/usr/local/bin/entrypoint.sh"]

# Expose the port that your backend listens on (adjust if needed)
EXPOSE 8080
