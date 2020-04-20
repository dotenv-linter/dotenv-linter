FROM rust:1.42 as builder
WORKDIR /usr/src
RUN rustup target add x86_64-unknown-linux-musl

RUN USER=root cargo new dotenv-linter
WORKDIR /usr/src/dotenv-linter
COPY Cargo.toml ./
RUN cargo build --release

COPY src ./src
RUN cargo install --target x86_64-unknown-linux-musl --path .

FROM scratch
COPY --from=builder /usr/local/cargo/bin/dotenv-linter /
ENTRYPOINT ["/dotenv-linter"]
