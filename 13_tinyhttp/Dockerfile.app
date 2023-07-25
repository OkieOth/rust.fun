FROM rust:latest as build

WORKDIR /usr/src/app

COPY . .

RUN cargo build --release

FROM debian:buster-slim

WORKDIR /usr/src/app

COPY --from=build /usr/src/app/target/release/my_http_server .

EXPOSE 8000

CMD ["./my_http_server"]
