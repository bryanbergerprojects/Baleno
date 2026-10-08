#!/usr/bin/env bash
# Installs the project toolchain and dependencies in one command.
# Prerequisites: proto and rustup.
set -euo pipefail

cd "$(dirname "$0")/.."

require() {
    local tool="$1" hint="$2"
    if ! command -v "$tool" >/dev/null 2>&1; then
        printf 'bootstrap: missing prerequisite "%s" (%s)\n' "$tool" "$hint" >&2
        exit 1
    fi
}

require proto "https://moonrepo.dev/docs/proto/install"
require rustup "https://rustup.rs"

# Tools installed by proto must win over any global node or pnpm.
proto_home="${PROTO_HOME:-$HOME/.proto}"
export PATH="$proto_home/shims:$proto_home/bin:$PATH"

proto install
# proto delegates Rust to rustup; this makes sure the channel from
# rust-toolchain.toml is complete, with its listed components.
rustup toolchain install
pnpm install
