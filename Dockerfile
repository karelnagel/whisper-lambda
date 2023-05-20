FROM rust:1.67-slim as builder

WORKDIR /build

ADD Cargo.toml Cargo.toml
ADD Cargo.lock Cargo.lock
ADD src src

RUN apt-get update && \
    apt-get install -y --no-install-recommends cmake make g++ libclang-dev libfindbin-libs-perl  && \
    rm -rf /var/lib/apt/lists/*

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/home/root/app/target \
    cargo build --release 


FROM public.ecr.aws/amazonlinux/amazonlinux:2023

COPY --from=builder /build/target/release/whisper /var/task/whisper

# Create the bootstrap script
RUN mkdir -p /var/runtime && \
    echo '#!/bin/sh' > /var/runtime/bootstrap && \
    echo 'exec /var/task/whisper' >> /var/runtime/bootstrap && \
    chmod +x /var/runtime/bootstrap

ENTRYPOINT ["/var/runtime/bootstrap"]