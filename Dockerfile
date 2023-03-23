FROM rust:latest AS builder
COPY . /
WORKDIR /

RUN cargo build --release

# use slim ubuntu build
FROM ubuntu:20.04 AS candidate

COPY --from=builder ./target/release ./target/release

# INSTALL SYSTEM DEPENDENCIES
RUN DEBIAN_FRONTEND=noninteractive apt-get update && apt-get install -y ca-certificates libssh-4 openssl libpq-dev libssl-dev libsasl2-dev curl && update-ca-certificates


CMD ["/target/release/url-condenser"]