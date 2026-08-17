# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.0] - 2026-08-17

### Changed — breaking

- `positive` 0.5 → 0.6 and `expiration_date` 0.2 → 0.3. Both appear in this
  crate's public API (`Positive` payloads on `Barrier`, `Bermuda`, `Rainbow`
  and friends; `ExpirationDate` on the exercise-date variants), so consumers
  must move to the same majors in one step. `positive` 0.6 is itself a
  breaking release: `ln`/`log10` return `Decimal`, serde emits the exact
  decimal as a string (`"42.5"` instead of `42.5`, old numeric documents
  still deserialise), `==` against `Decimal`/`f64` is exact rather than
  epsilon-based, and `PositiveError::Other` plus `new_unchecked` are gone.
- MSRV raised from 1.85 to 1.86, required by `expiration_date` 0.3. The
  `msrv` workflow now pins 1.86.

### Changed

- `utoipa` 5.4 → 5.5 and `criterion` 0.5 → 0.8 (dev-only). Every other
  dependency was already at its latest stable minor.

### Housekeeping

- `.cargo/audit.toml`, mirroring the `positive` crate's policy file: it ignores
  RUSTSEC-2026-0235 (rkyv 0.7.46) with a reachability rationale — `rkyv` is an
  *optional* dependency of `rust_decimal`, reached only transitively through
  `positive` and `expiration_date`, and neither enables that feature, so it is
  recorded in `Cargo.lock` but never compiled (`cargo tree --all-features
  --target all -i rkyv` reports nothing) — and opts into failing on
  unmaintained/unsound/notice advisories. The entry has an owner and a
  2027-02-15 review date.

## [0.2.0] - 2026-07-09

### Added

- Criterion benchmark harness under `benches/` (dev-only; `criterion` `0.5`
  with the `html_reports` feature added as a dev-dependency, so it never leaks
  into the published dependency tree). Three `harness = false` bench targets:
  `classification.rs` (every branch-free `is_*` helper — 5 on `OptionType`,
  4 on `BarrierType`, 2 on `AsianAveragingType`, 2 on `RainbowType`),
  `display.rs` (`Display`/`to_string` rendering for representative variants,
  including the payload-carrying `Barrier`, `Bermuda`, and `Rainbow`), and
  `serde.rs` (`serde_json` `to_string`/`from_str` round-trips per serializable
  variant family; `Compound` is omitted as it is `#[serde(skip)]`). Every input
  is `black_box`ed. A `make bench` target runs the suite, and the README
  documents the criterion baseline workflow (`--save-baseline` / `--baseline`).
- Coverage threshold gate via `codecov.yml`: project coverage target 80%
  with a 2-percentage-point drop tolerance, and 80% patch coverage for
  new/changed lines. PR comments require changed coverage. The policy is
  documented in `CONTRIBUTING.md`.

- `CONTRIBUTING.md` and `SECURITY.md` at the repository root. `CONTRIBUTING.md`
  documents the branching model (`issue-<n>-<slug>` branches, one issue per PR,
  `Closes #<n>`, merge commits), the `make pre-push` check gate, commit
  conventions, the generated-README workflow (`make readme`), the MSRV bump
  policy, CHANGELOG discipline, and semver rules. `SECURITY.md` documents
  private vulnerability reporting (GitHub Security Advisories or
  <jb@taunais.com>), the supported-versions table (latest published minor), and
  the disclosure timeline (acknowledgment within 7 days, fix or mitigation
  target within 90 days, coordinated disclosure). Linked both from the
  "Contribution and Contact" section of `README.tpl` / `README.md`.
- `cargo-readme` wiring to keep `README.md` generated from `src/lib.rs` crate
  docs + `README.tpl` instead of hand-maintained: new `make readme` target
  (regenerates `README.md`, auto-installing `cargo-readme` if absent, mirroring
  the `coverage` target's `cargo-tarpaulin` auto-install pattern) and
  `make readme-check` target (fails with a pointer to `make readme` when
  `README.md` drifts from the generated output). `make readme-check` is now
  part of `make pre-push`, and `.github/workflows/format_check.yml` runs it
  in CI after `fmt-check` so a stale `README.md` fails the build. Added a
  "Sub-type Enums" table (real variant names for `AsianAveragingType`,
  `BarrierType`, `BinaryType`, `LookbackType`, `RainbowType`) to the
  `src/lib.rs` crate docs so the regenerated `README.md` (and docs.rs) stay
  informative now that the old hand-written Overview/API body is gone.
- docs.rs metadata for full-feature documentation rendering:
  `[package.metadata.docs.rs]` with `all-features = true` and
  `rustdoc-args = ["--cfg", "docsrs"]`, plus `#![cfg_attr(docsrs,
  feature(doc_cfg))]` in `src/lib.rs`. The `doc_cfg` feature only takes
  effect when `#[doc(cfg(...))]` annotations are present; the crate has none
  today, because the `utoipa` feature gates only the `ToSchema` derive — no
  whole item exists solely under the feature — so enabling it is
  forward-preparation. Instead, a "Feature flags" section was added to
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

- **BREAKING:** Every numeric payload field on `OptionType` now uses the
  validated [`positive::Positive`] newtype instead of a raw `f64`, so each is
  guaranteed non-negative at construction. All nine fields migrated:

  | Variant.field | Old type | New type |
  |---|---|---|
  | `Bermuda.exercise_dates` | `Vec<f64>` | `Vec<Positive>` |
  | `Barrier.barrier_level` | `f64` | `Positive` |
  | `Barrier.rebate` | `Option<f64>` | `Option<Positive>` |
  | `Chooser.choice_date` | `f64` | `Positive` |
  | `Cliquet.reset_dates` | `Vec<f64>` | `Vec<Positive>` |
  | `Spread.second_asset` | `f64` | `Positive` |
  | `Exchange.second_asset` | `f64` | `Positive` |
  | `Quanto.exchange_rate` | `f64` | `Positive` |
  | `Power.exponent` | `f64` | `Positive` |

  The JSON wire shape is unchanged — every field still serializes to a plain
  JSON number — but a negative number is now **rejected at deserialization**
  (previously any `f64` was accepted). Non-self-describing binary formats
  (e.g. bincode) do see a different byte layout. Construct values with `positive`'s
  `pos!`/`pos_or_panic!`/`spos!` macros or `Positive::new`. As an additive
  bonus, with no floating-point field remaining `OptionType` now also derives
  `Eq` and `Hash` (it remains non-`Copy` due to the `Vec`/`Box` payloads and
  does not derive `Ord`). `Display` output for the affected variants changes
  where `f64` formatting differed from `Positive` (which normalizes decimals):
  e.g. `Some(5.0)` → `Some(5)` for `Barrier` rebates and
  `[30.0, 60.0, 90.0]` → `[30, 60, 90]` for `Bermuda`/`Cliquet` date lists.

- **BREAKING:** Every public enum is now `#[non_exhaustive]` — [`OptionType`]
  and the five leaf sub-enums (`AsianAveragingType`, `BarrierType`,
  `BinaryType`, `LookbackType`, `RainbowType`). This lets future variant
  additions ship as **minor** version bumps instead of major ones. Constructing
  existing variants is unaffected; the impact is on downstream `match`
  statements, which must now include a wildcard (`_`) arm. Migration — add a
  wildcard arm to any exhaustive match:

  ```rust
  // Before (0.1.x) — exhaustive match compiled fine:
  match option {
      OptionType::European => { /* ... */ }
      OptionType::American => { /* ... */ }
      // ... every remaining variant enumerated ...
  }

  // After (0.2.0) — add a wildcard arm:
  match option {
      OptionType::European => { /* ... */ }
      OptionType::American => { /* ... */ }
      _ => { /* handle current and future variants */ }
  }
  ```

- The `financial_types`, `positive`, and `expiration_date` dependencies no
  longer enable their `utoipa` feature unconditionally — it is now activated
  only through this crate's `utoipa` feature, matching the documented feature
  contract. Builds without the feature no longer compile the dependencies'
  `ToSchema` machinery. Note for downstream users who relied on the
  (undocumented) always-on activation: enable the `utoipa` feature
  explicitly. Full removal of `utoipa` from the no-feature dependency tree
  additionally requires an upstream `positive` release that stops hardwiring
  it.
- Standardized CI caching across all workflows on `Swatinem/rust-cache@v2`,
  replacing the hand-rolled `actions/cache@v4` blocks in `build.yml`,
  `code_coverage.yml`, `format_check.yml`, `lint.yml`, `msrv.yml`, and
  `release.yml`. The old keys were partly broken: `hashFiles('**/Cargo.lock')`
  hashed a file that isn't tracked in the repo (`Cargo.lock` is `.gitignore`d),
  so on a fresh checkout the key was effectively constant and never
  invalidated by dependency changes. `rust-cache` derives its key from
  `Cargo.toml`/workspace metadata and the resolved dependency graph instead,
  and (for `msrv.yml`) automatically separates the 1.85 toolchain's cache from
  the `stable` jobs' by keying on the compiler version, so the hand-rolled
  `-msrv-` key suffix is no longer needed. `build.yml`'s two-OS matrix now
  passes `key: ${{ matrix.os }}` to `rust-cache` so `ubuntu-22.04` and
  `ubuntu-latest` don't share a cache slot. `audit.yml` was left without
  `rust-cache`: `actions-rust-lang/audit@v1` only installs and runs
  `cargo-audit` against `Cargo.lock` — it never compiles this crate or its
  dependency graph, so a build-artifact cache has nothing to speed up (and
  the action already caches its own `cargo-audit` binary internally).
- Added a `concurrency: { group: ${{ github.workflow }}-${{ github.ref }},
  cancel-in-progress: true }` block to every workflow except `release.yml`,
  so superseded runs on the same branch/PR are cancelled instead of queuing
  (`code_coverage.yml` and `semver.yml` already had this and are unchanged).
  `release.yml` intentionally has no concurrency block — a release run must
  never be auto-cancelled mid-publish. `release.yml` still gained
  `rust-cache` since it runs the full pre-submission checklist
  (`clippy`, tests, `cargo build --release`, `cargo doc`) and caching only
  speeds up compilation there; it never affects which artifacts get
  published since `cargo publish` always builds from a fresh, verified
  package.

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
