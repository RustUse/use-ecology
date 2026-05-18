# Forges and Mirrors

The canonical repository for RustUse/use-ecology is GitHub:

```text
https://github.com/RustUse/use-ecology
```

Other forges may be used as public read-only mirrors, issue intake mirrors, or CI mirrors, but they do not have release authority unless maintainers update this document and [GOVERNANCE.md](GOVERNANCE.md).

## Release Authority

- crates.io publishes are made from the canonical GitHub repository.
- GitHub release tags and release artifacts are authoritative.
- Mirror CI can validate changes, but it does not publish crates.

## Mirror Sync

Maintainers can use `scripts/sync-mirrors.sh` to fetch and optionally push configured mirror remotes:

```sh
bash scripts/sync-mirrors.sh --dry-run
bash scripts/sync-mirrors.sh --push gitlab codeberg
```

The script defaults to all configured non-`origin` remotes when no remote names are passed.

## Contribution Provenance

When a contribution originates on a mirror, preserve authorship when porting it to the canonical repository. Link the mirrored discussion from the canonical pull request whenever possible.

## Security Reports

Security reports should follow the RustUse organization security policy on the canonical repository. Do not discuss unresolved vulnerabilities in public mirror issues unless maintainers have already disclosed them.
