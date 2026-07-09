#![cfg_attr(docsrs, feature(doc_cfg))]
//! # Option Type
//!
//! Option contract type definitions including standard and exotic options.
//!
//! This crate provides the [`OptionType`] enum which classifies options contracts
//! by their exercise characteristics and payoff structures:
//!
//! - **Standard**: European, American
//! - **Path-dependent**: Asian, Barrier, Lookback, Cliquet
//! - **Multi-asset**: Rainbow, Spread, Exchange
//! - **Structural**: Compound, Chooser, Binary, Bermuda
//! - **Modified payoff**: Power, Quanto
//!
//! It also provides sub-type enums used within exotic option variants:
//!
//! | Enum | Variants | Description |
//! |---|---|---|
//! | [`AsianAveragingType`] | `Arithmetic`, `Geometric` | Averaging method for Asian options |
//! | [`BarrierType`] | `UpAndIn`, `UpAndOut`, `DownAndIn`, `DownAndOut` | Barrier trigger conditions |
//! | [`BinaryType`] | `CashOrNothing`, `AssetOrNothing`, `Gap` | Binary option payout types |
//! | [`LookbackType`] | `FixedStrike`, `FloatingStrike` | Lookback strike determination |
//! | [`RainbowType`] | `BestOf`, `WorstOf` | Multi-asset selection method |
//!
//! And the lightweight [`OptionBasicType`] struct for referencing core option properties.
//!
//! All leaf enums use `#[repr(u8)]` for compact memory layout.
//! Pure helper methods are annotated with `#[must_use]` and `#[inline]`.
//!
//! Every numeric payload field on [`OptionType`] (barrier levels, rebates,
//! exercise/reset dates, asset prices, exchange rates, exponents) is a
//! validated [`positive::Positive`] rather than a raw `f64`, so it is
//! guaranteed non-negative at construction and rejected on deserialization
//! otherwise. Because no variant carries a floating-point field, [`OptionType`]
//! derives `Eq` and `Hash`.
//!
//! ## Features
//!
//! - Full `serde` serialization/deserialization support
//! - Optional `utoipa` support for OpenAPI schema generation (enable the `utoipa` feature)
//! - Depends on [`financial_types`] for `OptionStyle` and `Side`
//! - Depends on [`positive`] for `Positive` type-safe values
//! - Depends on [`expiration_date`] for `ExpirationDate`
//!
//! ## Feature flags
//!
//! - **`utoipa`** (off by default): adds a `utoipa::ToSchema` derivation to every
//!   public type — [`OptionType`], the leaf sub-enums, and [`OptionBasicType`] —
//!   so they can be embedded in OpenAPI schemas, and forwards the feature to the
//!   [`financial_types`], [`positive`], and [`expiration_date`] dependencies.
//!
//! Note that all public types exist in both configurations; the feature only adds
//! the `ToSchema` implementation. No item in this crate is gated behind `utoipa`,
//! so none carries a `doc(cfg)` badge on <https://docs.rs>.
//!
//! ## Usage
//!
//! ```rust
//! use option_type::{OptionType, AsianAveragingType, BarrierType};
//! use positive::pos_or_panic;
//!
//! let european = OptionType::European;
//! let asian = OptionType::Asian {
//!     averaging_type: AsianAveragingType::Arithmetic,
//! };
//! let barrier = OptionType::Barrier {
//!     barrier_type: BarrierType::UpAndIn,
//!     barrier_level: pos_or_panic!(120.0),
//!     rebate: None,
//! };
//!
//! assert_eq!(format!("{european}"), "European Option");
//! assert!(european.is_european());
//! assert!(asian.is_exotic());
//! assert!(barrier.is_path_dependent());
//! ```
//!
//! ## Non-exhaustive enums
//!
//! As of **0.2.0**, every public enum in this crate — [`OptionType`] and the
//! five leaf sub-enums ([`AsianAveragingType`], [`BarrierType`], [`BinaryType`],
//! [`LookbackType`], [`RainbowType`]) — is annotated `#[non_exhaustive]`.
//!
//! This lets the crate add new option families and sub-type variants in future
//! **minor** releases without a breaking major bump. The trade-off is on the
//! consumer side: an exhaustive `match` on any of these enums must include a
//! wildcard (`_`) arm, so that a newly added variant does not break your build.
//!
//! ```rust
//! use option_type::OptionType;
//! use positive::pos_or_panic;
//!
//! fn label(option: &OptionType) -> &'static str {
//!     match option {
//!         OptionType::European => "European",
//!         OptionType::American => "American",
//!         // Required: `OptionType` is `#[non_exhaustive]`, so new variants
//!         // may appear in a minor release. The wildcard keeps downstream
//!         // matches compiling across those additions.
//!         _ => "other",
//!     }
//! }
//!
//! assert_eq!(label(&OptionType::European), "European");
//! assert_eq!(label(&OptionType::Power { exponent: pos_or_panic!(2.0) }), "other");
//! ```
//!
//! Constructing existing variants is unaffected — you can still build any
//! variant with its normal literal syntax.

mod basic_type;
mod option_type;
pub mod prelude;
mod sub_enums;

pub use basic_type::OptionBasicType;
pub use option_type::OptionType;
pub use sub_enums::{AsianAveragingType, BarrierType, BinaryType, LookbackType, RainbowType};
