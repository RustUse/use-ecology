#!/usr/bin/env bash
set -euo pipefail

DRY_RUN=0
PUSH=0
REMOTE_NAMES=()

usage() {
  cat <<'EOF'
Usage: scripts/sync-mirrors.sh [--dry-run] [--push] [remote...]

Fetches configured git remotes and optionally pushes the current branch and tags
to named mirrors. With no remotes, all non-origin remotes are used.
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
    --push)
      PUSH=1
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      REMOTE_NAMES+=("$1")
      ;;
  esac
  shift
done

if ! git rev-parse --show-toplevel >/dev/null 2>&1; then
  echo "run from a git repository" >&2
  exit 1
fi

if [ "${#REMOTE_NAMES[@]}" -eq 0 ]; then
  while IFS= read -r remote; do
    [ "$remote" = "origin" ] && continue
    REMOTE_NAMES+=("$remote")
  done < <(git remote)
fi

if [ "${#REMOTE_NAMES[@]}" -eq 0 ]; then
  echo "no mirror remotes configured"
  exit 0
fi

for remote in "${REMOTE_NAMES[@]}"; do
  run_cmd git fetch --prune "$remote"
done

if [ "$PUSH" -eq 1 ]; then
  current_branch="$(git branch --show-current)"
  if [ -z "$current_branch" ]; then
    echo "cannot push mirrors from a detached HEAD" >&2
    exit 1
  fi

  for remote in "${REMOTE_NAMES[@]}"; do
    run_cmd git push "$remote" "HEAD:${current_branch}"
    run_cmd git push "$remote" --tags
  done
fi