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
//! - [`AsianAveragingType`] — Arithmetic or Geometric averaging
//! - [`BarrierType`] — Up/Down and In/Out barrier conditions
//! - [`BinaryType`] — Cash-or-nothing, Asset-or-nothing, Gap
//! - [`LookbackType`] — Fixed or Floating strike
//! - [`RainbowType`] — Best-of or Worst-of multi-asset
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
