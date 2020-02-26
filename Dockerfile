FROM rust:1.41 as builder
WORKDIR /app 
COPY . /app
RUN cargo build --release  && \
    chmod +x target/release/dotenv-linter

FROM scratch

COPY --from=0 /app/target/release/dotenv-linter /
ENTRYPOINT ["/dotenv-linter"]
