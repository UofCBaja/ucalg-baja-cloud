# ----------- Build Stage -----------
FROM rust:alpine AS builder
<<<<<<< HEAD

WORKDIR /ucalg-baja-cloud
    
=======

WORKDIR /ucalg_baja_cloud

>>>>>>> merch
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
RUN cargo build --release  && strip target/release/ucalg-baja-cloud
    
# ----------- Runtime Stage -----------
FROM alpine:latest
<<<<<<< HEAD

# Install runtime dependencies
RUN apk add --no-cache \
    bash \
    openssl \
    musl \
    libffi
    
WORKDIR /ucalg-baja-cloud
COPY --from=builder /ucalg-baja-cloud/target/release/ucalg-baja-cloud .
    
=======
    
# Install runtime dependencies
RUN apk add --no-cache \
    bash \
    openssl \
    musl \
    libffi
    
WORKDIR /ucalg_baja_cloud
COPY --from=builder /ucalg_baja_cloud/target/release/ucalg-baja-cloud .
    
>>>>>>> merch
EXPOSE 6525
CMD ["./ucalg-baja-cloud"]