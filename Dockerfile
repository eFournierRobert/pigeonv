FROM rust:slim-trixie AS builder

WORKDIR /app
COPY . .

RUN cargo build --release

FROM debian:trixie-slim

WORKDIR /app

COPY --from=builder /app/target/release/pigeonv .

RUN mkdir /app/db

EXPOSE 8080

CMD [ "./pigeonv" ]
