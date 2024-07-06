ARG RUNTIME_VERSION="alpine3.20"
FROM rust:${RUNTIME_VERSION}

RUN apk update && apk add --no-cache \
    pkgconfig \
    openssl-dev \
    build-base \
    clang \
    cmake \
    libffi-dev \
    linux-headers \
    musl-dev \
    openssl-libs-static

WORKDIR /application
COPY application/ .

WORKDIR /application/common
RUN cargo build --release 
WORKDIR /application/web
RUN cargo install --path . 

WORKDIR /application

RUN rustup component add rustfmt && \
    rustup component add clippy && \
    cargo install cargo-watch && \
    cargo install cargo-audit && \
    cargo install sqlx-cli