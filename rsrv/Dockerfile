FROM alpine as builder

WORKDIR /app

RUN apk update && apk add --no-cache curl libgcc build-base

ENV CARGO_HOME=/opt/rust
ENV RUSTUP_HOME=/opt/rust

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --no-modify-path --default-toolchain nightly

ADD Cargo.toml .
ADD ./src ./src

RUN \
source $CARGO_HOME/env &&\
mkdir -p out &&\
cargo build --release --out-dir out -Z unstable-options &&\
mv out/* main

FROM alpine

WORKDIR /app

COPY --from=builder /app/main main

EXPOSE 8080

ENV RUST_LOG info,supervisor=warn,hyper=warn,rustls=warn

ENTRYPOINT ["./main"]
