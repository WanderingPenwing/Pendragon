FROM mcr.microsoft.com/devcontainers/rust:latest

# Install additional tools
RUN apt-get update && apt-get install -y \
    libssl-dev pkg-config \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /workspaces/pendragon

# Pre-build the interpreter
RUN cargo build --release
