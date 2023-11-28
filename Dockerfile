# Build
FROM rust:1.74.0-buster as builder

WORKDIR /src

COPY . .

RUN cargo build --release

# Run
FROM debian:buster-slim

WORKDIR /app

COPY --from=builder /src/target/release/ushlepok .

CMD ["./ushlepok"]

EXPOSE 3000/tcp