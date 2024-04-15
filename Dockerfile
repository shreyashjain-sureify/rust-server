FROM rust:latest AS builder

WORKDIR /usr/src/rust-server

# Copy the dependency manifests
COPY Cargo.toml Cargo.lock ./

# Build dependencies
RUN cargo fetch

COPY . .

# Build the project
RUN cargo build --release

# Smaller base image for the final runtime environment
FROM debian:buster-slim

WORKDIR /usr/src/rust-server

# Copy the built binary from the builder stage
COPY --from=builder /usr/src/rust-server/target/release/server .

EXPOSE 8085

# Command to run the server
CMD ["./rust-server"]
