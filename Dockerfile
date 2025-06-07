# Use the official Rust image as the base image
FROM rust:1.86 AS builder

# Set the working directory
WORKDIR /usr/src/work-pulse

# Copy the Cargo.toml and Cargo.lock files
COPY src/Cargo.toml src/Cargo.lock ./

# Copy the source code
COPY src/work-pulse-core ./work-pulse-core
COPY src/work-pulse-service ./work-pulse-service

# Build the application
RUN cargo build --workspace --release

# Use a smaller base image for the final image
FROM debian:trixie-slim

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/work-pulse/target/release/work-pulse-service /usr/local/bin/work-pulse-service

EXPOSE 8080

# Set the command to run the application
CMD ["work-pulse-service"]