# Changelog

Notable changes to `mini-functions`. Earlier releases are described on
the [GitHub releases page](https://github.com/sebastienrousseau/mini-functions/releases).

## [0.0.12] - Unreleased

### Changed

- **Breaking: the MSRV is now Rust 1.89.0.** The root crate and the
  workspace stated 1.71.1, `cclm` and `mdg` 1.67, and `cjwt` and `idk`
  1.66.1; every manifest and the README now state 1.89.0, one floor
  shared with the maintainer's other Rust crates.
- `deny.toml` uses the current cargo-deny schema. cargo-deny 0.20
  refused the file (`unlicensed`, `copyleft`, `allow-osi-fsf-free` and
  the licence `deny` list are gone), so `cargo deny check` could not
  run. The allow list now names the licences the old rules accepted in
  the dependency graph; the policy is unchanged.
- CI runs the shared pipelines security workflow (cargo audit, cargo
  deny, CodeQL and dependency review), as the other crates in the
  family do. cargo deny ran nowhere in CI before.
