# Build time argument to include CA certificates
ARG INCLUDE_CA=false

# Use the official Rust image as the base image
FROM rust:1.86 AS builder

# Set the working directory
WORKDIR /usr/src/work-pulse

# Copy the Cargo.toml and Cargo.lock files
COPY src/Cargo.docker.toml ./Cargo.toml
COPY src/Cargo.lock ./

# Copy the source code
COPY src/work-pulse-core ./work-pulse-core
COPY src/work-pulse-service ./work-pulse-service

RUN mkdir -p ./certificates
COPY certificates ./certificates

# Copy CA certificate inside the container and install the SSL certificates (if applicable)
RUN if [ "$INCLUDE_CA" = "true" ] && [ -d "./certificates" ]; then \
      cp -r ./certificates /usr/local/share/ca-certificates && \
      apt-get update && apt-get install -y ca-certificates && \
      update-ca-certificates; \
    fi

# Build the application
RUN cargo build --package work-pulse-service --release

# Use a smaller base image for the final image
FROM debian:trixie-slim

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/work-pulse/target/release/work-pulse-service /usr/local/bin/work-pulse-service

EXPOSE 8080

# Set the command to run the application
CMD ["work-pulse-service"]