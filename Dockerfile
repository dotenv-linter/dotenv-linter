FROM rust:latest as builder
WORKDIR /usr/src
RUN rustup target add x86_64-unknown-linux-musl

RUN USER=root cargo new dotenv-linter
WORKDIR /usr/src/dotenv-linter
COPY Cargo.toml ./
COPY src ./src
COPY benches ./benches

RUN cargo build --release
RUN cargo install --target x86_64-unknown-linux-musl --path .

FROM scratch
COPY --from=builder /usr/local/cargo/bin/dotenv-linter /
ENTRYPOINT ["/dotenv-linter"]
