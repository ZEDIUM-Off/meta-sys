#!/usr/bin/env bash

set -euo pipefail

repo_root="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo_root"

package_count="$(cargo metadata --format-version 1 --no-deps | \
    sed -n 's/.*"packages":\[\([^]]*\)\].*/\1/p')"

if [[ -n "$package_count" ]]; then
    cargo fmt --all --check
    cargo check --workspace --all-targets
    cargo clippy --workspace --all-targets -- -D warnings
    cargo test --workspace --all-targets
    cargo test --workspace --doc
    cargo doc --workspace --no-deps
    cargo dylint --all -- --workspace --all-targets
else
    echo "No product crate yet; skipping product compilation and lint gates."
fi

(
    cd tools/dylint/meta_sys_style
    cargo fmt -- --check
    cargo test
    cargo clippy --all-targets -- -D warnings
)
