# Use a Rust base image
FROM rust:latest as builder

# Create a new empty shell project
RUN USER=root cargo new --bin my_app
WORKDIR /my_app

# Copy your manifests
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock

# This build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# Now that the dependencies are built, copy your source code
COPY ./src ./src

# Build for release.
RUN rm ./target/release/deps/my_app*
RUN cargo build --release

# Final base image
FROM debian:buster-slim
RUN apt-get update && apt-get install -y openssl && rm -rf /var/lib/apt/lists/*

# Copy the build artifact from the build stage and set the binary as the entrypoint
COPY --from=builder /my_app/target/release/my_app /usr/local/bin

# Set the startup command to run your binary
CMD ["my_app"]
