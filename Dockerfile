FROM rust:1.41 as builder
WORKDIR /app 
COPY . /app
RUN cargo build --release  && \
    chmod +x target/release/dotenv-linter

FROM scratch

COPY --from=builder /app/target/release/dotenv-linter /
ENTRYPOINT ["/dotenv-linter"]
