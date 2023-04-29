FROM docker.io/rust:1.69 AS builder

WORKDIR /app
COPY . .

WORKDIR /app/eight-serve
RUN cargo build --release

FROM debian:bullseye-slim

COPY --from=builder /app/eight-serve/target/release/eight-serve /release/eight-serve
ENTRYPOINT [ "/release/eight-serve" ]