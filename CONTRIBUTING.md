# Contributing

RustUse/use-ecology is intentionally small. Contributions should favor durable vocabulary, clear naming, strong documentation, and minimal surface area over broad feature count.

For routing and organization-wide policy, use the RustUse defaults for [support](https://github.com/RustUse/.github/blob/main/SUPPORT.md), [security](https://github.com/RustUse/.github/blob/main/SECURITY.md), and the [code of conduct](https://github.com/RustUse/.github/blob/main/CODE_OF_CONDUCT.md), alongside `GOVERNANCE.md` and `MAINTAINERS.md`.

## Development Flow

1. Make the smallest useful change that improves the current crates.
2. Add or update unit tests for every public function or type behavior you introduce or change.
3. Keep dependencies lightweight unless there is a strong justification.
4. Preserve the primitive-vocabulary boundary: no ecology simulator, no population dynamics engine, no biodiversity database, no GIS surface, no conservation-planning system, and no prediction or external-data behavior.

## Local Validation

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

## Tooling Shortcuts

The repository ships cross-platform Cargo aliases in `.cargo/config.toml`:

```sh
cargo xcheck
cargo xlint
cargo xtest
cargo xtest-minimal
cargo xexamples
cargo xdoc
```

These shortcuts let contributors use the repo-owned validation path without depending on `make`, which keeps local workflows friendlier on Windows and other environments where `cargo` is available but GNU Make may not be.

VS Code users also get checked-in task definitions in `.vscode/tasks.json` and extension recommendations in `.vscode/extensions.json`.

## Optional Dev Tool Bootstrap

Optional Cargo tooling used by local release and advisory flows can be installed with either bootstrap script:

```sh
bash scripts/bootstrap-dev-tools.sh
pwsh -File scripts/bootstrap-dev-tools.ps1
```

These scripts install `cargo-deny`, `cargo-audit`, `cargo-cyclonedx`, `release-plz`, and `cargo-machete`.

## Documentation

- Update the root README when the crate list or facade story changes.
- Keep crate README examples small and runnable.
- Keep docs aligned with the current ecology scope and non-goals.
- Follow `CRATE_TEMPLATE.md` when introducing a new focused crate or expanding the facade surface.

## Release Policy

- The workspace-level default keeps `publish = false`, while current crate manifests opt in with `publish = true`.
- Publish focused crates before the `use-ecology` facade.
- Versions move in lockstep at `0.x.y` for now.
- Until `1.0`, breaking API changes should bump the minor version and compatible additive changes should bump the patch version.
- `Cargo.lock` is committed intentionally for reproducible CI, security checks, and release dry runs in this library workspace.

`release-plz` drives release PRs and changelog generation for publishable crates. Prefer commit subjects that match `type: summary` or `type(scope)!: summary`, such as `feat: add habitat condition primitive`, `fix: reject empty ecosystem names`, or `docs: clarify ecology boundaries`.
