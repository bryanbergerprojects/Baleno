#!/usr/bin/env bash
# Enforces the dependency rule: only adapters and binaries may depend on an
# HTTP framework, SQLx, a Docker or PTY client, or another adapter. Every
# other workspace member is checked, so a new crate is checked by default
# until listed below.
# Usage: check-dependency-rule.sh [crate...]   (default: every member)
set -euo pipefail

cd "$(dirname "$0")/.."

# Adapters and binaries: exempt from the check. Classifying a crate as an
# adapter is a deliberate edit of this list.
exempt=(
    baleno baleno-agent http-api sqlite agent-hub
)

# Crates that a checked member must never reach through normal dependencies.
# Contexts reach agents through the FleetGateway port, never the wire format.
forbidden=(
    axum axum-core hyper sqlx tower-http bollard portable-pty
    http-api sqlite agent-hub agent-protocol
)

contains() {
    local needle="$1" item
    shift
    for item in "$@"; do
        [[ "$item" == "$needle" ]] && return 0
    done
    return 1
}

metadata="$(cargo metadata --no-deps --format-version 1)"
members=()
while IFS= read -r name; do members+=("$name"); done < <(jq -r '.packages[].name' <<<"$metadata")

if (($# > 0)); then
    targets=("$@")
else
    targets=("${members[@]}")
fi

status=0
for crate in "${targets[@]}"; do
    contains "$crate" "${members[@]}" || {
        printf 'dependency-rule: "%s" is not a workspace member\n' "$crate" >&2
        exit 2
    }
    contains "$crate" "${exempt[@]}" && continue

    pkgid="$(jq -r --arg name "$crate" '.packages[] | select(.name == $name) | .id' <<<"$metadata")"
    # Normal edges only, on every target: dev and build dependencies are
    # allowed, platform-specific ones are not overlooked.
    while IFS= read -r dep; do
        if contains "$dep" "${forbidden[@]}"; then
            printf 'dependency-rule: "%s" depends on forbidden crate "%s"\n' "$crate" "$dep" >&2
            cargo tree -p "$pkgid" -e normal --target all -i "$dep" >&2 || true
            status=1
        fi
    done < <(
        cargo tree -p "$pkgid" -e normal --target all --prefix none --format '{p}' \
            | tail -n +2 | awk '{print $1}' | sort -u
    )
done

exit "$status"
