FROM messense/rust-musl-cross:x86_64-musl AS builder-amd64
FROM messense/rust-musl-cross:aarch64-musl AS builder-arm64

# hadolint ignore=DL3006
FROM builder-${TARGETARCH} AS builder

ARG TARGETARCH

WORKDIR /

COPY Cargo.toml Cargo.lock ./
COPY dotenv-lookup ./dotenv-lookup
COPY dotenv-linter ./dotenv-linter

RUN if [ "${TARGETARCH}" = "amd64" ]; \
  then export TARGET="x86_64-unknown-linux-musl"; \
  else export TARGET="aarch64-unknown-linux-musl"; \
  fi \
  && cargo install --target "${TARGET}" --path dotenv-linter

FROM scratch
COPY --from=builder /root/.cargo/bin/dotenv-linter /
ENTRYPOINT ["/dotenv-linter"]
