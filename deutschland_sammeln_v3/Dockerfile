# This Dockerfile currently uses the default dockerfile from the leptos book for leptos-ssr deployement
# If you’re using stable, use this instead
FROM rust:1.88-bookworm as builder

# Install cargo-binstall, which makes it easier to install other
# cargo extensions like cargo-leptos
RUN wget https://github.com/cargo-bins/cargo-binstall/releases/latest/download/cargo-binstall-aarch64-unknown-linux-musl.tgz
RUN tar -xvf cargo-binstall-aarch64-unknown-linux-musl.tgz
RUN cp cargo-binstall /usr/local/cargo/bin

# Install required tools
RUN apt-get update -y \
  && apt-get install -y --no-install-recommends clang

# Install cargo-sqlx
# RUN cargo binstall sqlx-cli -y

# Install cargo-leptos
RUN cargo binstall cargo-leptos -y

# Add the WASM target
RUN rustup target add wasm32-unknown-unknown

# Make an /app dir, which everything will eventually live in
RUN mkdir -p /app
WORKDIR /app
COPY . .

# Build the app
ENV LEPTOS_WASM_OPT_VERSION=version_124
ENV SQLX_OFFLINE=true
# ENV DATABASE_URL=postgresql://localhost:5432/deutschland_sammeln?user=deutschland_sammeln_bot&password=wHz6ghG6-Fa$8uu3
# ENV PGUSER=deutschland_sammeln_bot
# ENV PGHOST=localhost
# ENV PGDATABASE=deutschland_sammeln
# ENV PGPASSWORD=wHz6ghG6-Fa$8uu3
# ENV PGPORT=5432
# RUN cargo sqlx prepare
RUN cargo leptos build --release -vv

FROM debian:bookworm-slim as runtime
WORKDIR /app
RUN apt-get update -y \
  && apt-get install -y --no-install-recommends openssl ca-certificates \
  && apt-get autoremove -y \
  && apt-get clean -y \
  && rm -rf /var/lib/apt/lists/*

# -- NB: update binary name from "leptos_start" to match your app name in Cargo.toml --
# Copy the server binary to the /app directory
COPY --from=builder /app/target/release/deutschland_sammeln_v3 /app/

# /target/site contains our JS/WASM/CSS, etc.
COPY --from=builder /app/target/site /app/site

# Copy Cargo.toml if it’s needed at runtime
COPY --from=builder /app/Cargo.toml /app/

COPY db_config.toml /app/

# Set any required env variables and
ENV LIVE_CERT_PEM="/servers.dasmonitor.ovh/fullchain.pem"
ENV LIVE_KEY_PEM="/servers.dasmonitor.ovh/privkey.pem"
ENV RUST_LOG="info"
ENV LEPTOS_SITE_ADDR="0.0.0.0:8080"
ENV LEPTOS_SITE_ROOT="site"
ENV OUTPUT-NAME="Online Sammeln"
EXPOSE 8080

# -- NB: update binary name from "leptos_start" to match your app name in Cargo.toml --
# Run the server
CMD ["/app/deutschland_sammeln_v3"]
