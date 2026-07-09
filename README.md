[![Dual License](https://img.shields.io/badge/license-MIT-blue)](./LICENSE)
[![Crates.io](https://img.shields.io/crates/v/option_type.svg)](https://crates.io/crates/option_type)
[![Downloads](https://img.shields.io/crates/d/option_type.svg)](https://crates.io/crates/option_type)
[![Stars](https://img.shields.io/github/stars/joaquinbejar/option_type.svg)](https://github.com/joaquinbejar/option_type/stargazers)
[![Issues](https://img.shields.io/github/issues/joaquinbejar/option_type.svg)](https://github.com/joaquinbejar/option_type/issues)
[![PRs](https://img.shields.io/github/issues-pr/joaquinbejar/option_type.svg)](https://github.com/joaquinbejar/option_type/pulls)
[![Build Status](https://img.shields.io/github/workflow/status/joaquinbejar/option_type/CI)](https://github.com/joaquinbejar/option_type/actions)
[![Coverage](https://img.shields.io/codecov/c/github/joaquinbejar/option_type)](https://codecov.io/gh/joaquinbejar/option_type)
[![Dependencies](https://img.shields.io/librariesio/github/joaquinbejar/option_type)](https://libraries.io/github/joaquinbejar/option_type)
[![Documentation](https://img.shields.io/badge/docs-latest-blue.svg)](https://docs.rs/option_type)
[![Wiki](https://img.shields.io/badge/wiki-latest-blue.svg)](https://deepwiki.com/joaquinbejar/option_type)


## Option Type

Option contract type definitions including standard and exotic options.

This crate provides the [`OptionType`] enum which classifies options contracts
by their exercise characteristics and payoff structures:

- **Standard**: European, American
- **Path-dependent**: Asian, Barrier, Lookback, Cliquet
- **Multi-asset**: Rainbow, Spread, Exchange
- **Structural**: Compound, Chooser, Binary, Bermuda
- **Modified payoff**: Power, Quanto

It also provides sub-type enums used within exotic option variants:

| Enum | Variants | Description |
|---|---|---|
| [`AsianAveragingType`] | `Arithmetic`, `Geometric` | Averaging method for Asian options |
| [`BarrierType`] | `UpAndIn`, `UpAndOut`, `DownAndIn`, `DownAndOut` | Barrier trigger conditions |
| [`BinaryType`] | `CashOrNothing`, `AssetOrNothing`, `Gap` | Binary option payout types |
| [`LookbackType`] | `FixedStrike`, `FloatingStrike` | Lookback strike determination |
| [`RainbowType`] | `BestOf`, `WorstOf` | Multi-asset selection method |

And the lightweight [`OptionBasicType`] struct for referencing core option properties.

All leaf enums use `#[repr(u8)]` for compact memory layout.
Pure helper methods are annotated with `#[must_use]` and `#[inline]`.

### Features

- Full `serde` serialization/deserialization support
- Optional `utoipa` support for OpenAPI schema generation (enable the `utoipa` feature)
- Depends on [`financial_types`] for `OptionStyle` and `Side`
- Depends on [`positive`] for `Positive` type-safe values
- Depends on [`expiration_date`] for `ExpirationDate`

### Feature flags

- **`utoipa`** (off by default): adds a `utoipa::ToSchema` derivation to every
  public type — [`OptionType`], the leaf sub-enums, and [`OptionBasicType`] —
  so they can be embedded in OpenAPI schemas, and forwards the feature to the
  [`financial_types`], [`positive`], and [`expiration_date`] dependencies.

Note that all public types exist in both configurations; the feature only adds
the `ToSchema` implementation. No item in this crate is gated behind `utoipa`,
so none carries a `doc(cfg)` badge on <https://docs.rs>.

### Usage

```rust
use option_type::{OptionType, AsianAveragingType, BarrierType};

let european = OptionType::European;
let asian = OptionType::Asian {
    averaging_type: AsianAveragingType::Arithmetic,
};
let barrier = OptionType::Barrier {
    barrier_type: BarrierType::UpAndIn,
    barrier_level: 120.0,
    rebate: None,
};

assert_eq!(format!("{european}"), "European Option");
assert!(european.is_european());
assert!(asian.is_exotic());
assert!(barrier.is_path_dependent());
```


## Examples

Runnable examples live under [`examples/`](./examples). Each prints a short,
self-explanatory walkthrough of one part of the API:

| Example | Description | Command |
|---|---|---|
| `basic_usage` | Construct European/American options, query the `is_*` helpers, print `Display` labels | `cargo run --example basic_usage` |
| `exotic_options` | Build Asian, Barrier, Lookback, Rainbow, and Bermuda options with realistic payloads and print their classification | `cargo run --example exotic_options` |
| `serde_roundtrip` | Serialize a variety of variants to JSON and deserialize them back, printing the JSON | `cargo run --example serde_roundtrip` |
| `option_basic_type` | Assemble an `OptionBasicType` from `OptionStyle`, `Side`, `Positive`, and `ExpirationDate`, printing its borrowed fields | `cargo run --example option_basic_type` |


## Benchmarks

The crate ships a [criterion](https://crates.io/crates/criterion) harness under
[`benches/`](./benches), grouped into three benchmark files:

| Bench | Covers |
|---|---|
| `classification` | Every branch-free `is_*` helper on `OptionType`, `BarrierType`, `AsianAveragingType`, and `RainbowType` |
| `display` | `Display` (`to_string`) rendering for representative variants, including payload-carrying `Barrier`, `Bermuda`, and `Rainbow` |
| `serde` | `serde_json` `to_string` / `from_str` round-trips per serializable variant family |

Run the full suite with:

```bash
make bench          # or: cargo bench
```

To track performance across changes, save a criterion baseline and compare
against it later (using the current version as the baseline name):

```bash
cargo bench -- --save-baseline v0.1.2   # record a baseline
cargo bench -- --baseline v0.1.2        # compare against it
```


## MSRV

The Minimum Supported Rust Version (MSRV) is **1.85** (required by `edition = "2024"`
and by this crate's direct dependencies). CI enforces the MSRV via a dedicated
`msrv.yml` workflow that builds and tests the crate on the pinned toolchain.

MSRV bumps are considered breaking changes and require a major (or pre-1.0
minor) version bump.


## Releasing

Releases are tag-driven:

1. Bump `version` in `Cargo.toml`; promote `[Unreleased]` in `CHANGELOG.md`
   to a dated `## [X.Y.Z]` section.
2. Ensure `make pre-push` is clean, then run `make tag` and
   `git push origin vX.Y.Z`.
3. `.github/workflows/release.yml` validates the tag against `Cargo.toml`,
   runs the full check matrix, publishes to crates.io, and creates the
   GitHub Release from the matching `CHANGELOG.md` section.


## Contribution and Contact

We welcome contributions to this project! Please read
[CONTRIBUTING.md](https://github.com/joaquinbejar/option_type/blob/main/CONTRIBUTING.md) for the full workflow — branching model,
the `make pre-push` check gate, commit conventions, README regeneration, and
semver discipline — then:

1. Fork the repository.
2. Create a new branch for your feature or bug fix.
3. Make your changes and ensure that the project still builds and all tests pass.
4. Commit your changes and push your branch to your forked repository.
5. Submit a pull request to the main repository.

Found a security vulnerability? Please **do not** open a public issue — follow
the private reporting process in [SECURITY.md](https://github.com/joaquinbejar/option_type/blob/main/SECURITY.md).

If you have any questions, issues, or would like to provide feedback, please feel free to contact the project maintainer:


### **Contact Information**

- **Author**: Joaquín Béjar García
- **Email**: jb@taunais.com
- **Telegram**: [@joaquin_bejar](https://t.me/joaquin_bejar)
- **Repository**: <https://github.com/joaquinbejar/option_type>
- **Documentation**: <https://docs.rs/option_type>

We appreciate your interest and look forward to your contributions!

## ✍️ License

Licensed under **MIT** license
