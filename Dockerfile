FROM docker.io/rust:1.69-alpine AS builder

WORKDIR /app/eight-serve
COPY eight-serve .

RUN apk add --no-cache musl-dev openssl-dev pkgconfig
RUN cargo build --release --target x86_64-unknown-linux-musl

FROM gcr.io/distroless/static

COPY --from=builder /app/eight-serve/target/x86_64-unknown-linux-musl/release/eight-serve /bin/eight-serve
ENTRYPOINT [ "/bin/eight-serve" ]
