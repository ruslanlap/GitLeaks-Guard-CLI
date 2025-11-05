# Dockerfile: use official Rust image to avoid rustup/apt issues on runner
FROM rust:latest

# Install system deps required by project (adjust if necessary)
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
      build-essential git curl ca-certificates pkg-config libssl-dev && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /work

# Cache dependencies: copy manifest and fetch
# Note: cargo fetch may fail during build due to SSL cert issues in build env,
# but dependencies will be fetched during actual build in CI environment
COPY Cargo.toml Cargo.lock ./
RUN mkdir -p src && echo "fn main(){}" > src/main.rs && \
    (cargo fetch || echo "Pre-fetch failed, will fetch during build")

# Copy project files
COPY . .

# Default command: run a shell so CI can execute commands
CMD ["/bin/bash"]
