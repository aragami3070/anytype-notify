FROM rust:latest

WORKDIR /app

COPY . .

RUN mkdir -p /app/assets

RUN cargo build --release

CMD ["./scripts/start.sh"]
