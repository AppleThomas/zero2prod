# Rust stable release
FROM rust:1.70.0

# Change working directory to 'app
WORKDIR /app

# install required system dependencies for our linking configuration
RUN apt update && apt install lld clang -y

# Copy all files from our working environment to the Docker image
COPY . .

# build vinary and use the release profile to make it fast
RUN cargo build --release

# Launch the binary when docker run is executed
ENTRYPOINT ["./target/release/zero2prod"]