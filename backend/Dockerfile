# Builder Stage
FROM rust:1.73 as builder

# Set working directory inside the container
WORKDIR /app

# Copy Cargo files first to leverage Docker's layer caching
COPY Cargo.toml Cargo.lock ./

# Create a dummy `main.rs` to precompile dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Precompile dependencies
RUN cargo build --release

# Remove the dummy `main.rs` to make way for actual source code
RUN rm -rf src

# Copy the actual source code
COPY ./src ./src

# Build the application
RUN cargo build --release

# Runtime Stage
FROM debian:buster-slim

# Install required runtime dependencies (e.g., PostgreSQL client libraries)
RUN apt-get update && apt-get install -y libpq-dev && rm -rf /var/lib/apt/lists/*

# Set working directory for the runtime container
WORKDIR /app

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release/backend .

# Expose the application port
EXPOSE 8080

# Run the application
CMD ["backend"]
