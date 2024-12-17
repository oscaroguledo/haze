# Use the Rust image
FROM rust:1.73 as builder

# Set the working directory
WORKDIR /app

# Copy dependencies files
COPY Cargo.toml Cargo.lock ./

# Build the dependencies (caches dependencies)
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# Copy the actual source code
COPY ./src ./src

# Compile the backend application
RUN cargo build --release

# Runtime stage
FROM debian:buster-slim

# Install necessary libraries for PostgreSQL
RUN apt-get update && apt-get install -y libpq-dev && rm -rf /var/lib/apt/lists/*

COPY ./migrations ./migrations
RUN cargo install sqlx-cli --features postgres
RUN sqlx migrate run

# Copy the binary from the builder stage
COPY --from=builder /app/target/release/backend .

# Expose the application port
EXPOSE 8080

# Run the application
CMD ["./backend"]
