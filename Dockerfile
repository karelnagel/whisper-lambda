FROM rust:1.67 as builder

WORKDIR /build

ADD Cargo.toml Cargo.toml
ADD Cargo.lock Cargo.lock
ADD src src

RUN apt-get update
RUN apt-get install -y  cmake libclang-dev
RUN cargo install cargo-lambda 

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/home/root/app/target \
    rustup target add aarch64-unknown-linux-gnu && \
    cargo lambda build --release --target aarch64-unknown-linux-gnu --compiler cargo

RUN ls /build/target/lambda/whisper

FROM public.ecr.aws/amazonlinux/amazonlinux:2023
COPY --from=builder /build/target/lambda/whisper/bootstrap /bootstrap

ENTRYPOINT ["/bootstrap"]