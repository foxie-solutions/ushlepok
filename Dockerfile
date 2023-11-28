# Build
FROM rust:1.74.0-buster as builder

WORKDIR /src

COPY . .

RUN cargo build --release

# Run
FROM alpine:3.18.4

WORKDIR /app

COPY --from=builder /src/target/release/ushlepok .

CMD ["/app/ushlepok"]

EXPOSE 3000/tcp