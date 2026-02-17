FROM rust:1.76 as builder

WORKDIR /app
COPY . .
COPY ../engine ../engine

RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /app
COPY --from=builder /app/target/release/password_api .

EXPOSE 8080
CMD ["./password_api"]
