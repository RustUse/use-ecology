# Maintainer Release Flow

This document describes how maintainers should run releases with the current `release-plz` setup.

## Current Model

- `Release PR Automation` opens or updates a release PR from `main`.
- `release-plz` keeps every publishable crate in the workspace in one lockstep version group.
- The shared root `CHANGELOG.md` is generated through the `use-ecology` package entry and includes focused-crate commits.
- `Release Publish Automation` runs automatically on pushes to `main` after the initial manual publish wave is complete and the repository enables the guarded auto-publish path.

## One-Time Post-Initial-Release Setup

- Configure crates.io Trusted Publishing for every published crate with repository owner `RustUse`, repository name `use-ecology`, and workflow filename `release-plz-release.yml`.
- Set the repository variable `CRATES_IO_AUTOPUBLISH_ENABLED` to `true` only after the first manual crates.io wave is complete.

## Normal Post-Initial-Release Flow

1. Merge ordinary PRs into `main` with clean conventional-commit style in the final commit subject or squash-merge title.
2. Let `Release PR Automation` open or update the release PR.
3. Review the lockstep version bump and generated `CHANGELOG.md`.
4. Merge the release PR after the required checks pass.
5. Let `Release Publish Automation` publish from the merged release commit, or manually dispatch it with `post-initial-release = true` if a controlled rerun is needed.

## Initial Public Release Exception

Do not use `Release Publish Automation` for the first public crates.io wave.

1. Confirm the intended first-wave publishable crates are still `use-ecosystem`, `use-habitat`, `use-population`, `use-community`, `use-niche`, `use-biome`, `use-food-web`, `use-species-interaction`, `use-trophic-level`, `use-biodiversity`, `use-conservation-status`, and `use-ecology`.
2. Run the full publish-readiness checks.
3. Dry-run and then publish focused crates in the order documented in [RELEASE.md](../RELEASE.md).
4. Wait for crates.io index propagation.
5. Run `cargo publish --dry-run -p use-ecology` or the manual `Facade Publish Readiness` workflow.
6. Publish `use-ecology`.
