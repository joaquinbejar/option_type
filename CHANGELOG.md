# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Integration test suite under `tests/` exercising the crate through its
  public surface only: `serde_roundtrip.rs` (round-trip of every serializable
  enum variant), `display.rs` (exact `Display` labels), `helpers.rs` (every
  `is_*` classification helper against its truth table), `basic_type.rs`
  (`OptionBasicType` construction and `Copy`/`Clone`/`Eq`/`Hash`/`Debug`
  semantics), and `utoipa.rs` (`utoipa`-gated schema generation).

### Changed

- Split `src/lib.rs` into focused modules (`option_type`, `sub_enums/*`,
  `basic_type`); crate root now holds docs and re-exports only. Pure
  refactor — public API unchanged.

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
