ARG RUNTIME_VERSION="alpine3.20"
FROM rust:${RUNTIME_VERSION}

RUN apk update && apk add --no-cache musl-dev gcc

WORKDIR /application
COPY application/ .

WORKDIR /application/common
RUN cargo build --release 
WORKDIR /application/web
RUN cargo install --path . 

WORKDIR /application

RUN rustup component add rustfmt && rustup component add clippy && cargo install cargo-watch && cargo install cargo-audit