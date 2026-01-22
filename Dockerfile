# Get started with a build env with Rust nightly
FROM rustlang/rust:nightly-bookworm AS builder

# If you’re using stable, use this instead
# FROM rust:1.88-bookworm as builder

# Install cargo-binstall, which makes it easier to install other
# cargo extensions like cargo-leptos
RUN wget https://github.com/cargo-bins/cargo-binstall/releases/latest/download/cargo-binstall-aarch64-unknown-linux-musl.tgz
RUN tar -xvf cargo-binstall-aarch64-unknown-linux-musl.tgz
RUN cp cargo-binstall /usr/local/cargo/bin

# Install required tools
RUN apt-get update -y \
  && apt-get install -y --no-install-recommends clang openssl ca-certificates

# Install cargo-leptos
RUN cargo binstall cargo-leptos -y

# Add the WASM target
RUN rustup target add wasm32-unknown-unknown

# manually install tailwindcss v3.4.19 to /usr/local/bin/tailwindcss
RUN wget https://github.com/tailwindlabs/tailwindcss/releases/download/v3.4.19/tailwindcss-linux-arm64 -O /usr/local/bin/tailwindcss \
    && chmod +x /usr/local/bin/tailwindcss

# Force Tailwind v3 to avoid v4 breaking changes & Setup cache so cargo-leptos doesn't download it
ENV LEPTOS_TAILWIND_VERSION="v3.4.19"
RUN mkdir -p /root/.cache/cargo-leptos/tailwindcss-v3.4.19/ \
    && cp /usr/local/bin/tailwindcss /root/.cache/cargo-leptos/tailwindcss-v3.4.19/tailwindcss-linux-arm64 \
    && chmod +x /root/.cache/cargo-leptos/tailwindcss-v3.4.19/tailwindcss-linux-arm64
# Make an /app dir, which everything will eventually live in
RUN mkdir -p /app
WORKDIR /app
COPY . .

# Build the app
RUN cargo leptos build --release -vv

FROM debian:bookworm-slim AS runtime
WORKDIR /app
RUN apt-get update -y \
  && apt-get install -y --no-install-recommends openssl ca-certificates \
  && apt-get autoremove -y \
  && apt-get clean -y \
  && rm -rf /var/lib/apt/lists/*

# Copy the server binary to the /app directory
COPY --from=builder /app/target/release/temidaradev-rust /app/

# /target/site contains our JS/WASM/CSS, etc.
COPY --from=builder /app/site /app/site

# Copy Cargo.toml if it’s needed at runtime
COPY --from=builder /app/Cargo.toml /app/

# Set any required env variables and
ENV RUST_LOG="info"
ENV LEPTOS_SITE_ADDR="0.0.0.0:8080"
ENV LEPTOS_SITE_ROOT="site"
EXPOSE 8080

# Run the server
CMD ["/app/temidaradev-rust"]
