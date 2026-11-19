#!/usr/bin/env bash
set -euo pipefail

# input.css and tailwind.config.js use Tailwind 3.
export LEPTOS_TAILWIND_VERSION=v3.4.19
cargo leptos build --release \
  --lib-cargo-args='--locked' \
  --bin-cargo-args='--locked' "$@"
