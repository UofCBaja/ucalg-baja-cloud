# ----------- Build Stage -----------
FROM rust:alpine AS builder

WORKDIR /ucalg-baja-cloud
    
# Install build dependencies
RUN apk add --no-cache \
    pkgconfig \
    musl-dev \
    openssl-dev \
    bash \
    make \
    g++ \
    cmake \
    libffi-dev
    
# Copy source and build
COPY . .
RUN cargo build --release
    
# ----------- Runtime Stage -----------
FROM alpine:latest

# Install runtime dependencies
RUN apk add --no-cache \
    bash \
    openssl \
    musl \
    libffi
    
WORKDIR /ucalg-baja-cloud
COPY --from=builder /ucalg-baja-cloud/target/release/ucalg-baja-cloud .
    
EXPOSE 6525
CMD ["./ucalg-baja-cloud"]