FROM rust:1.53 as builder
RUN cargo new --bin actix-micro
WORKDIR ./actix-micro
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release

RUN rm src/*.rs
ADD . ./
RUN cargo build --release 



FROM ubuntu:latest
RUN apt-get -y update
RUN apt-get -y upgrade
RUN apt-get install -y libmariadb-dev
COPY --from=builder ./actix-micro/target/release/actix-microservice /usr/local/bin
COPY ./.dockerenv /root/.env
WORKDIR /root
CMD ["actix-microservice"]

