# 1. Use Rust official image
FROM rust:1.76 as builder

# 2. Set working directory to /app/api
WORKDIR /app/api

# 3. Copy API code
COPY ./api ./api
# Copy the engine as dependency
COPY ../engine ../engine

# 4. Build the API in release mode
RUN cargo build --release

# 5. Use minimal runtime image
FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /app/api/target/release/password_api .

EXPOSE 8080
CMD ["./password_api"]
