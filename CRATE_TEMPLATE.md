# RustUse Crate Template

Use this checklist when adding a new focused crate or expanding the `use-ecology` facade.

## Target Layout

```text
crates/use-example/
  Cargo.toml
  README.md
  src/
    lib.rs
```

Examples, integration tests, `prelude.rs`, and `error.rs` are optional. Keep the default shape small unless the crate needs more structure.

## Cargo.toml Pattern

```toml
[package]
name = "use-example"
description = "Primitive example vocabulary for RustUse"
publish = true
version.workspace = true
authors.workspace = true
edition.workspace = true
license.workspace = true
repository.workspace = true
homepage.workspace = true
rust-version.workspace = true
readme = "README.md"
documentation = "https://docs.rs/use-example"
keywords = ["ecology", "vocabulary", "rustuse"]
categories = ["data-structures", "science"]

[package.metadata.docs.rs]
all-features = true

[lints]
workspace = true
```

Checklist:

- Keep package metadata inherited from the workspace wherever possible.
- Keep dependencies absent unless the crate composes another focused RustUse primitive.
- Use a primitive-vocabulary description that matches the existing crate wording.
- Keep the crate out of simulation, GIS, biodiversity analytics, conservation planning, and external-data domains unless a future set explicitly owns that scope.

## src/lib.rs Pattern

```rust
#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;
```

Checklist:

- Re-export the focused public API at the crate root.
- Prefer small validated constructors for caller-provided text.
- Preserve original name text except for documented trimming of surrounding whitespace.
- Implement `Display` and `FromStr` where stable vocabulary parsing is useful.
- Avoid macros, async, global runtime assumptions, and unsafe code.

## README Structure

Keep crate README files short and consistent:

- title and one-line primitive-vocabulary summary
- explicit scope boundary and non-goals
- small runnable rustdoc example

## Facade Checklist

If a new focused crate should be available through `use-ecology`, also update:

- root `Cargo.toml` workspace members, workspace dependencies, and first-publish metadata
- `crates/use-ecology/Cargo.toml` dependencies
- `crates/use-ecology/src/lib.rs` module aliases and prelude
- `crates/use-ecology/README.md`
- root `README.md`
- `release-plz.toml`, `Makefile`, `.vscode/tasks.json`, and publish-readiness workflows

## Testing Checklist

- Add unit tests for each public constructor, parse/display behavior, validation rule, and collection invariant.
- Add doctest examples in crate READMEs.
- Keep examples descriptive. They should not analyze, infer, simulate, fetch, map, plan, or resolve external ecology data.

## Validation Checklist

```sh
cargo fmt --all -- --check
cargo check --workspace --all-features
cargo check --workspace --all-features --examples
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo deny check
cargo audit
cargo test --workspace --all-features
cargo test --workspace --no-default-features
cargo doc --workspace --all-features --no-deps
```
