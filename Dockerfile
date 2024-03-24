# syntax=docker/dockerfile:1
FROM rust:alpine as builder
RUN apk add --no-cache build-base

# Don't download the entire crates.io package index. Fetch only the index
# entries for crates that are actually used. This is faster and avoids a memory
# usage explosion that often breaks docker builds.
# https://blog.rust-lang.org/inside-rust/2023/01/30/cargo-sparse-protocol.html#background
ENV CARGO_REGISTRIES_CRATES_IO_PROTOCOL="sparse"

WORKDIR /app/

COPY Cargo.toml Cargo.lock ./
COPY src src

RUN cargo build --release && \
    cp /app/target/release/merino /app/merino

###############################################################################
## Final image
###############################################################################
FROM alpine:3.19

ENV USER=app \
    UID=10001

RUN apk add --update --no-cache \
            tzdata~=2024 && \
    rm -rf /var/cache/apk && \
    rm -rf /var/lib/app/lists*


RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/${USER}" \
    --shell "/sbin/nologin" \
    --uid "${UID}" \
    "${USER}" && \
    chown -R app:app /app

WORKDIR /app
USER merino
COPY --from=builder /app/merino /app/

CMD ["/app/merino"]
