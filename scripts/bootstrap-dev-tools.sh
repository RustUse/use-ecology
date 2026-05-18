#!/usr/bin/env bash
set -euo pipefail

TOOLS=(cargo-deny cargo-audit cargo-cyclonedx release-plz cargo-machete)
DRY_RUN=0

usage() {
  cat <<'EOF'
Usage: scripts/bootstrap-dev-tools.sh [--dry-run]

Installs the optional Rust workspace tooling used by local validation,
release automation, and advisory checks.
EOF
}

run_cmd() {
  if [ "$DRY_RUN" -eq 1 ]; then
    printf '+'
    printf ' %q' "$@"
    printf '\n'
  else
    "$@"
  fi
}

while [ "$#" -gt 0 ]; do
  case "$1" in
    --dry-run)
      DRY_RUN=1
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "unknown argument: $1" >&2
      usage >&2
      exit 2
      ;;
  esac
  shift
done

if ! command -v cargo >/dev/null 2>&1; then
  echo "cargo is required" >&2
  exit 1
fi

rustup component add rustfmt clippy

for tool in "${TOOLS[@]}"; do
  echo "Installing or updating ${tool}"
  run_cmd cargo install --locked "$tool"
done

echo "Optional RustUse development tools are ready."