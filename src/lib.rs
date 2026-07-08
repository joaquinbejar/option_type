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
//!
//! let european = OptionType::European;
//! let asian = OptionType::Asian {
//!     averaging_type: AsianAveragingType::Arithmetic,
//! };
//! let barrier = OptionType::Barrier {
//!     barrier_type: BarrierType::UpAndIn,
//!     barrier_level: 120.0,
//!     rebate: None,
//! };
//!
//! assert_eq!(format!("{european}"), "European Option");
//! assert!(european.is_european());
//! assert!(asian.is_exotic());
//! ```

mod basic_type;
mod option_type;
pub mod prelude;
mod sub_enums;

pub use basic_type::OptionBasicType;
pub use option_type::OptionType;
pub use sub_enums::{AsianAveragingType, BarrierType, BinaryType, LookbackType, RainbowType};
