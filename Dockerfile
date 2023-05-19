FROM amazonlinux:2023 as builder

WORKDIR /build

ADD Cargo.toml Cargo.toml
ADD Cargo.lock Cargo.lock
ADD src src

RUN yum update

# Cmake for whisper.cpp
RUN yum install -y cmake

# Install Rust 
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Install cargo-lambda
RUN cargo install cargo-lambda 

#
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/home/root/app/target \
    rustup target add aarch64-unknown-linux-gnu && \
    cargo lambda build --release --target aarch64-unknown-linux-gnu --compiler cargo

FROM amazonlinux:2023
COPY --from=builder /build/target/lambda/whisper/bootstrap /bootstrap

ENTRYPOINT ["/bootstrap"]