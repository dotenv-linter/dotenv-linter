FROM messense/rust-musl-cross:x86_64-musl AS builder

WORKDIR /

RUN cargo new dotenv-linter
COPY Cargo.toml ./
COPY dotenv-lookup dotenv-lookup
COPY dotenv-linter/Cargo.toml dotenv-linter/Cargo.toml
COPY dotenv-linter/src dotenv-linter/src
COPY dotenv-linter/benches dotenv-linter/benches
COPY .cargo .cargo

RUN cargo build --release \
 && cargo install --target x86_64-unknown-linux-musl --path dotenv-linter

FROM scratch
COPY --from=builder /root/.cargo/bin/dotenv-linter /
ENTRYPOINT ["/dotenv-linter"]
