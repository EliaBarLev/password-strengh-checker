# Stage 1: Build
FROM rust:1.76 AS builder

# Set working directory
WORKDIR /app

# Copy API and Engine folders into /app
COPY ./api ./api
COPY ./engine ./engine

# Build API in release mode
WORKDIR /app/api
RUN cargo build --release

# Stage 2: Minimal runtime
FROM debian:bookworm-slim
WORKDIR /app

# Copy the compiled binary
COPY --from=builder /app/api/target/release/password_api .

EXPOSE 8080
CMD ["./password_api"]
