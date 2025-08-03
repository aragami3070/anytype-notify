FROM rust:1.88.0-alpine3.22 AS builder

RUN apk add --no-cache musl-dev perl make pkgconfig openssl-libs-static openssl-dev
RUN apk add --no-cache --upgrade bash
WORKDIR /app

COPY Cargo.toml .
COPY Cargo.lock .
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release --locked --target x86_64-unknown-linux-musl

COPY . .
RUN touch src/main.rs
RUN cargo build --locked --release --target x86_64-unknown-linux-musl

FROM alpine:latest

WORKDIR /app
RUN mkdir -p /app/assets

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/anytype-notify /app/anytype-notify

COPY scripts/start.sh /app/start.sh
COPY config.toml /app/config.toml

RUN chmod +x /app/anytype-notify /app/start.sh

CMD ["sh", "start.sh"]
