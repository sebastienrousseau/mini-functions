# Changelog

Notable changes to `mini-functions`. Earlier releases are described on
the [GitHub releases page](https://github.com/sebastienrousseau/mini-functions/releases).

## [0.0.12] - 2026-11-06

The final release. `cclm`, `cjwt`, `idk` and `mdg` ship as 0.0.2 with it.

### Deprecated

- **`mini-functions` and its sub-crates `cclm`, `cjwt`, `idk` and `mdg`
  are deprecated and unmaintained.** The GitHub repository is archived
  after this release. Nothing on crates.io depends on any of them
  outside this workspace. Use `cmn`, `dtt`, `hsh`, `qrc`, `rlg` and
  `vrd` directly; for JSON Web Tokens use `jsonwebtoken`; for MD5 use
  `md-5`. Every manifest declares
  `[badges] maintenance = { status = "deprecated" }`, and every README
  and crate root opens with the notice.

### Fixed

- **`cjwt::JWT::validate` always returns an error and is
  `#[deprecated]`.** It never verified a token: it compared the HMAC of
  the whole token with the HMAC of the signature text, so a correctly
  signed token was rejected, and it panicked on a token with one dot.
  It also decoded with standard base64 rather than base64url and never
  checked `exp`. It is not repaired; use `jsonwebtoken`. The v0.0.11
  release notes describe a fix for the panic that is not in the v0.0.11
  tag; this release is the first that does not panic.

### Security

- `idk` and `cjwt` no longer declare the unmaintained `jwt` crate
  (RUSTSEC-2016-0007), which neither used.
- `idk` and `cjwt` require `ctutils` 0.4.4 or newer, so the versions
  affected by RUSTSEC-2026-0385 (reached through `hmac` and `digest`)
  cannot resolve.

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
- The release workflow builds with `cross build` directly instead of
  the archived `actions-rs/cargo` action, whose node12 runner GitHub no
  longer runs.
- The release changelog step writes the version heading, the
  template and the commit subjects in one redirect. The heading used to
  be appended and then overwritten by the template, so release notes
  started without it.
