# Builder stage
FROM rust:1.73 as builder

# Set the working directory
WORKDIR /app

# Copy dependency files
COPY Cargo.toml Cargo.lock ./
RUN cargo update
# Pre-build dependencies (cache dependencies)

RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# Install sqlx-cli for running migrations
RUN cargo install sqlx-cli --features postgres

# Copy application source code
COPY ./src ./src

# Copy migrations
COPY ./migrations ./migrations

# Set the database URL (adjust as needed)
ENV DATABASE_URL=postgres://postgres:password@db:5432/mydatabase

# Run database migrations during the build process
RUN sqlx migrate run

# Build the backend binary
RUN cargo build --release

# Runtime stage
FROM debian:buster-slim

# Install necessary libraries for PostgreSQL
RUN apt-get update && apt-get install -y libpq-dev && rm -rf /var/lib/apt/lists/*

# Copy the binary from the builder stage
COPY --from=builder /app/target/release/backend .

# Expose the application port
EXPOSE 8080

# Run the application
CMD ["./backend"]
