# Releasing

This repository uses a first-wave release flow for a multi-crate RustUse workspace.

## Current Release State

`use-ecology` publishes focused crates before the `use-ecology` facade crate.

## Canonical Release Guide

Use [RELEASE.md](RELEASE.md) as the authoritative release policy for:

- first-wave publish scope
- focused-crate publish ordering
- publish readiness checks
- trusted publishing setup after the first public wave
- maintainer release checklist

## Current Automation

The repository includes workflows that match this release shape:

- `publish-readiness.yml`
- `facade-publish-readiness.yml`
- `release-plz-pr.yml`
- `release-plz-release.yml`

This file exists to keep the top-level release entrypoint consistent with other RustUse repositories.
