# ----------- Build Stage -----------
FROM rust:1.9-slim AS builder

WORKDIR /ucalg_baja_cloud
    
# Install build dependencies
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*
    
# Copy source and build
COPY . .
RUN cargo build --release  && strip target/release/ucalg-baja-cloud
    
# ----------- Runtime Stage -----------
FROM debian:bookworm-slim
    
# Install runtime dependencies (e.g., for OpenSSL if needed)
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*
    
WORKDIR /ucalg_baja_cloud
COPY --from=builder /ucalg_baja_cloud/target/release/ucalg-baja-cloud .
    
EXPOSE 8000
CMD ["./ucalg-baja-cloud"]