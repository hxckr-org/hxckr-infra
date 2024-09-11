# Use the official Rust image as the base image
FROM rust:1.76 as builder

# Create a new empty shell project
RUN USER=root cargo new --bin git-service
WORKDIR /git-service

# Copy our manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# Build only the dependencies to cache them
RUN cargo build --release
RUN rm src/*.rs

# Copy the source code
COPY ./src ./src

# Build for release
RUN rm ./target/release/deps/git_service*
RUN cargo build --release

# Final stage
FROM ubuntu:22.04

# Install OpenSSL and ca-certificates
RUN apt-get update && \
    apt-get install -y openssl ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Copy the build artifact from the builder stage
COPY --from=builder /git-service/target/release/git-service .

# Set the startup command
CMD ["./git-service"]