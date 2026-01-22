FROM rustlang/rust:nightly-bookworm as builder

RUN apt-get update -y && \
    apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
    curl \
    git \
    build-essential \
    nodejs \
    npm

RUN wget https://github.com/cargo-bins/cargo-binstall/releases/latest/download/cargo-binstall-aarch64-unknown-linux-musl.tgz
RUN tar -xvf cargo-binstall-aarch64-unknown-linux-musl.tgz
RUN cp cargo-binstall /usr/local/cargo/bin

RUN apt-get update -y \
  && apt-get install -y --no-install-recommends clang

RUN cargo binstall cargo-leptos -y

RUN rustup target add wasm32-unknown-unknown

WORKDIR /app
COPY . .
RUN npm ci

RUN cargo leptos build --release -vv

FROM debian:bookworm-slim

WORKDIR /app
RUN apt-get update -y && \
    apt-get install -y --no-install-recommends openssl ca-certificates && \
    apt-get autoremove -y && \
    apt-get clean -y && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/temidaradev-rust /app/temidaradev-rust

COPY --from=builder /app/target/site /app/site

COPY --from=builder /app/.env /app/.env


EXPOSE 3004

CMD ["./temidaradev-rust"]
