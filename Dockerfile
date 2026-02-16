# Dockerfile for Linera Service on Railway/Render
FROM rust:1.86-slim

# Install dependencies
RUN apt-get update && apt-get install -y \
    build-essential \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Install Linera CLI
RUN cargo install linera-service@0.15.8 --locked

# Create app directory
WORKDIR /app

# Copy wallet and storage
COPY .linera /app/.linera

# Set environment variables
ENV LINERA_WALLET=/app/.linera/wallet.json
ENV LINERA_STORAGE=rocksdb:/app/.linera/storage.db
ENV PORT=8081

# Expose port
EXPOSE 8081

# Start Linera service
CMD ["linera", "service", "--port", "8081", "--listen-address", "0.0.0.0:8081"]
