# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- docs.rs metadata for full-feature documentation rendering:
  `[package.metadata.docs.rs]` with `all-features = true` and
  `rustdoc-args = ["--cfg", "docsrs"]`, plus `#![cfg_attr(docsrs,
  feature(doc_cfg))]` in `src/lib.rs` so docs.rs (which builds on nightly)
  can render feature badges. Because the `utoipa` feature gates only the
  `ToSchema` derive — no whole item exists solely under the feature — no
  `doc(cfg)` badges are emitted; instead a "Feature flags" section was added to
  the crate docs (`src/lib.rs`) and mirrored in `README.md` to document what the
  `utoipa` feature adds. The `docsrs` cfg is inert on stable and does not trigger
  `unexpected_cfgs` (rustc allowlists it in check-cfg), so no `check-cfg`
  declaration is required.
- Declared the Minimum Supported Rust Version: `rust-version = "1.85"` in
  `Cargo.toml` (required by `edition = "2024"` and by this crate's direct
  dependencies — `financial_types` and `expiration_date` both declare
  `rust-version = "1.85"`). Verified empirically with
  `cargo +1.85 build --all-features` and `cargo +1.85 test --all-features`.
  Added a dedicated `.github/workflows/msrv.yml` CI job that builds and tests
  (`--all-features` and `--no-default-features`) on the pinned MSRV toolchain.
  Documented the MSRV and its bump policy in `README.md` / `README.tpl`: MSRV
  bumps are breaking changes and require a major (or pre-1.0 minor) version
  bump.
- Runnable examples under `examples/`: `basic_usage.rs` (standard option types
  and classification helpers), `exotic_options.rs` (Asian, Barrier, Lookback,
  Rainbow, and Bermuda variants with realistic payloads), `serde_roundtrip.rs`
  (JSON serialize/deserialize of a variety of variants), and
  `option_basic_type.rs` (assembling `OptionBasicType` from its borrowed
  primitive fields). Each is invoked via `cargo run --example <name>`.
- Integration test suite under `tests/` exercising the crate through its
  public surface only: `serde_roundtrip.rs` (round-trip of every serializable
  enum variant), `display.rs` (exact `Display` labels), `helpers.rs` (every
  `is_*` classification helper against its truth table), `basic_type.rs`
  (`OptionBasicType` construction and `Copy`/`Clone`/`Eq`/`Hash`/`Debug`
  semantics), and `utoipa.rs` (`utoipa`-gated schema generation).
- Tag-driven release workflow (`.github/workflows/release.yml`), triggered on
  `v*.*.*` tag pushes: validates the tag against the `Cargo.toml` version and
  extracts the matching `CHANGELOG.md` section as release notes (failing
  before any check or publish if either is missing/mismatched), runs the
  full pre-submission checklist (`fmt --check`, `clippy -D warnings`, tests
  with `--all-features` and `--no-default-features`, `build --release`,
  `doc` with `RUSTDOCFLAGS=-D warnings`), then `cargo publish` to crates.io,
  and finally creates the GitHub Release from the extracted notes. Requires
  a `CARGO_REGISTRY_TOKEN` repository secret. Documented the release flow in
  `README.md` / `README.tpl` under a new "Releasing" section.

### Changed

- Split `src/lib.rs` into focused modules (`option_type`, `sub_enums/*`,
  `basic_type`); crate root now holds docs and re-exports only. Pure
  refactor — public API unchanged.
- Tightened `make pre-push` into a full check gate covering the
  pre-submission checklist: it now runs `fmt-check`, `lint`, `test`
  (all-features), the new `test-default` (`--no-default-features`), a
  `release` build, and the new `doc-check` (`RUSTDOCFLAGS="-D warnings"
  cargo doc --no-deps --all-features`). It is a pure check gate — the
  auto-fix targets (`fix`, `lint-fix`) are no longer invoked by `pre-push`
  and remain available standalone. Split `publish` into
  `publish-dry` (dry-run, safe to re-run) and `publish` (real
  `cargo publish`, aborts when neither `CARGO_REGISTRY_TOKEN` nor cargo
  login credentials are available). Added a self-documenting `help` target
  listing every Make target.

### Fixed

- CI `format_check` job now runs `make fmt-check`
  (`cargo +stable fmt --all --check`) instead of the auto-formatting
  `make fmt`, so formatting drift actually fails the job.

## [0.1.2] - 2026-04-15

### Changed

- Upgrade published dependency versions: `financial_types` `0.1` → `0.2` and
  `expiration_date` `0.1.2` → `0.2`.

## [0.1.1] - 2026-04-15

### Changed

- Upgrade `positive` `0.4` → `0.5` and bump `expiration_date` to `0.1.2`
  (which also uses `positive` `0.5`), eliminating the duplicate `positive`
  version Cargo was otherwise forced to keep in the dependency tree.

## [0.1.0] - 2026-02-20

Initial public release of `option_type`.

### Added

- `OptionType` enum classifying financial option contracts across standard
  and exotic families: `European`, `American`, `Bermuda`, `Asian`,
  `Barrier`, `Lookback`, `Cliquet`, `Rainbow`, `Spread`, `Exchange`,
  `Compound`, `Chooser`, `Binary`, `Power`, `Quanto`.
- Leaf sub-enums with `#[repr(u8)]` compact layout (1 byte each):
  `AsianAveragingType` (Arithmetic, Geometric), `BarrierType`
  (UpAndIn, UpAndOut, DownAndIn, DownAndOut), `BinaryType`
  (CashOrNothing, AssetOrNothing, Gap), `LookbackType` (FixedStrike,
  FloatingStrike), `RainbowType` (BestOf, WorstOf).
- Pure helper methods on `OptionType`: `is_european`, `is_american`,
  `is_exotic`, `is_path_dependent`, `is_multi_asset`.
- Helper methods on leaf sub-enums: `BarrierType::is_knock_in`,
  `is_knock_out`, `is_up`, `is_down`; `AsianAveragingType::is_arithmetic`,
  `is_geometric`; `RainbowType::is_best_of`, `is_worst_of`.
- `OptionBasicType` lightweight reference struct for core option
  properties (`option_style`, `side`, `strike_price`, `expiration_date`).
- `Display` impl for `OptionType` producing human-readable labels
  (e.g. `"European Option"`).
- `serde::Serialize` / `Deserialize` support on every public type.
- Optional `utoipa` feature exposing `ToSchema` derivations for OpenAPI
  schema generation, propagated to `financial_types`, `positive`, and
  `expiration_date` via their respective `utoipa` features.
- Crate-level prelude (`option_type::prelude`) re-exporting the public
  surface for `use option_type::prelude::*;`.
- `#[must_use]` on every pure helper method and `#[inline]` on hot-path
  accessors.
- Crate-level `[lints]` section denying `unwrap_used`, `expect_used`,
  `panic`, and `unsafe_code`; warning on `indexing_slicing`,
  `needless_collect`, `unnecessary_to_owned`, `clone_on_ref_ptr`,
  `missing_errors_doc`, `missing_panics_doc`, and `missing_docs`.
- `overflow-checks = true` in both `dev` and `release` profiles;
  thin LTO and single codegen-unit in `release`.
