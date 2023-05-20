FROM rust:1.67 as builder

WORKDIR /build

ADD Cargo.toml Cargo.toml
ADD Cargo.lock Cargo.lock
ADD src src

RUN apt-get update
RUN apt-get install -y  cmake libclang-dev

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/home/root/app/target \
    cargo build --release 

RUN ls /build/target/release

FROM public.ecr.aws/amazonlinux/amazonlinux:2023

COPY --from=builder /build/target/release/whisper /var/task/whisper

# Create the bootstrap script
RUN mkdir -p /var/runtime && \
    echo '#!/bin/sh' > /var/runtime/bootstrap && \
    echo 'exec /var/task/whisper' >> /var/runtime/bootstrap && \
    chmod +x /var/runtime/bootstrap

ENTRYPOINT ["/var/runtime/bootstrap"]