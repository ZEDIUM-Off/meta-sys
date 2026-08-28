#!/usr/bin/env bash

set -euo pipefail

repo_root="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo_root"

usage() {
    echo "Usage:" >&2
    echo "  $0 all" >&2
    echo "  $0 doc <package>" >&2
    echo "  $0 package <package>" >&2
    echo "  $0 one <package> <fully-qualified-test-name>" >&2
    exit 2
}

workspace_has_packages() {
    local packages
    packages="$(cargo metadata --format-version 1 --no-deps | \
        sed -n 's/.*"packages":\[\([^]]*\)\].*/\1/p')"
    [[ -n "$packages" ]]
}

run_all() {
    if workspace_has_packages; then
        cargo test --workspace --all-targets
        cargo test --workspace --doc
    else
        echo "No product crate yet; skipping product test suites."
    fi

    (cd tools/dylint/meta_sys_style && cargo test)
}

run_package() {
    local package="${1:-}"
    [[ -n "$package" ]] || usage

    cargo test --package "$package" --all-targets
    cargo test --package "$package" --doc
}

run_doc() {
    local package="${1:-}"
    [[ -n "$package" ]] || usage

    cargo test --package "$package" --doc
}

run_one() {
    local package="${1:-}"
    local test_name="${2:-}"
    [[ -n "$package" && -n "$test_name" ]] || usage

    local test_listing
    test_listing="$(cargo test --package "$package" --all-targets -- --list)"
    if ! awk -F ': ' -v name="$test_name" \
        '$1 == name && $2 == "test" { found = 1 } END { exit !found }' \
        <<<"$test_listing"; then
        echo "No test named '$test_name' found in package '$package'." >&2
        exit 2
    fi

    cargo test --package "$package" --all-targets "$test_name" -- --exact --nocapture
}

case "${1:-}" in
    all) run_all ;;
    doc) run_doc "${2:-}" ;;
    package) run_package "${2:-}" ;;
    one) run_one "${2:-}" "${3:-}" ;;
    *) usage ;;
esac
