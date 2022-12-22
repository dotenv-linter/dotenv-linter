FROM messense/rust-musl-cross:x86_64-musl as builder

WORKDIR /

RUN cargo new dotenv-linter
COPY Cargo.toml ./
COPY src src
COPY benches benches
COPY .cargo .cargo

RUN cargo build --release
RUN cargo install --target x86_64-unknown-linux-musl --path .

FROM scratch
COPY --from=builder /root/.cargo/bin/dotenv-linter /
ENTRYPOINT ["/dotenv-linter"]
