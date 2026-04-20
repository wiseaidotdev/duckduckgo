FROM rust:alpine AS builder

LABEL maintainer="Mahmoud Harmouch <oss@wiseai.dev>"
RUN apk update && apk upgrade && \
    apk add --no-cache \
    build-base \
    pkgconfig \
    openssl-dev \
    openssl-libs-static \
    git

WORKDIR /duckduckgo
COPY Cargo.toml Cargo.lock README.md ./
COPY src ./src
COPY ./RUST.md ./
COPY ./WASM.md ./

RUN cargo build --release --features="rust-binary"

FROM alpine:3.22.4

RUN apk add --no-cache openssl ca-certificates sudo && \
    addgroup -S duckduckgo && \
    adduser -S -G duckduckgo duckduckgo

RUN addgroup -S sudo && \
    addgroup duckduckgo sudo && \
    echo "%sudo ALL=(ALL) NOPASSWD:ALL" >> /etc/sudoers

WORKDIR /home/duckduckgo

COPY --from=builder /duckduckgo/target/release/ddg /usr/local/bin/ddg

USER duckduckgo

ENTRYPOINT [ "/usr/local/bin/ddg" ]
