FROM rust:1.99-bookworm AS builder

WORKDIR /build
COPY . .

ENV SQLX_OFFLINE=true

RUN --mount=type=cache,target=/build/target \
    --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    set -eux; \
    cargo build --release; \
    objcopy --compress-debug-sections target/release/deutschland_sammeln_online_V3_backend ./main

################################################################################

FROM debian:bookworm-slim as runtime

WORKDIR /app

## copy the main binary
COPY --from=builder /build/main /app/

## copy runtime assets which may or may not exist
COPY --from=builder /build/Rocket.tom[l] ./static
COPY --from=builder /build/stati[c] ./static
COPY --from=builder /build/template[s] ./templates

COPY db_config.toml /app/

## ensure the container listens globally on port 8080
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=7000

CMD ./main