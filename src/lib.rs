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

pub mod prelude;

use expiration_date::ExpirationDate;
use financial_types::{OptionStyle, Side};
use positive::Positive;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Represents the type of option in a financial context.
///
/// Options can be categorized into various types based on their characteristics
/// and the conditions under which they can be exercised. This enum covers both
/// standard (European, American) and exotic option types.
///
/// Note: `#[repr(u8)]` is **not** applied here because several variants carry
/// data fields. Leaf sub-enums (`BarrierType`, `BinaryType`, etc.) do use
/// `#[repr(u8)]`.
///
/// # Examples
///
/// ```rust
/// use option_type::OptionType;
///
/// let opt = OptionType::default();
/// assert!(opt.is_european());
/// assert!(!opt.is_exotic());
/// ```
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum OptionType {
    /// A European option can only be exercised at the expiry date.
    /// European options are simpler to price and analyze because their payoff
    /// is only determined at a single point in time.
    #[default]
    European,

    /// An American option can be exercised at any time before and including the
    /// expiry date. This provides the holder with more flexibility compared to
    /// European options.
    American,

    /// A Bermuda option can be exercised on specific dates before the expiry date.
    /// These dates are usually predetermined and occur at regular intervals
    /// (e.g., monthly or quarterly).
    Bermuda {
        /// The specific dates (in days to expiry) on which the option can be
        /// exercised before expiry.
        exercise_dates: Vec<f64>,
    },

    /// An Asian option whose payoff depends on the average price of the
    /// underlying asset over a certain period.
    Asian {
        /// The method used to calculate the average price (arithmetic or geometric).
        averaging_type: AsianAveragingType,
    },

    /// A Barrier option that becomes active or inactive only if the underlying
    /// asset reaches a certain barrier level.
    Barrier {
        /// The type of barrier that triggers the option's activation or deactivation.
        barrier_type: BarrierType,
        /// The price level that the underlying asset must reach for the barrier
        /// to be triggered.
        barrier_level: f64,
        /// The amount paid to the option holder if the option is knocked out.
        rebate: Option<f64>,
    },

    /// A Binary option that provides a fixed payoff if the underlying asset is
    /// above or below a certain level at expiry.
    Binary {
        /// The specific type of binary option.
        binary_type: BinaryType,
    },

    /// A Lookback option allowing the holder to "look back" over time and
    /// determine the payoff based on the maximum or minimum underlying asset
    /// price during the option's life.
    Lookback {
        /// The specific type of lookback option.
        lookback_type: LookbackType,
    },

    /// A Compound option that has an option as its underlying asset.
    #[serde(skip)]
    #[cfg_attr(feature = "utoipa", schema(skip))]
    Compound {
        /// The underlying option, which can be any type of option.
        underlying_option: Box<OptionType>,
    },

    /// A Chooser option allowing the holder to choose, at a certain date,
    /// whether the option will be a call or a put.
    Chooser {
        /// The date (in days to expiry) on which the holder must choose
        /// whether the option becomes a call or a put.
        choice_date: f64,
    },

    /// A Cliquet (ratchet) option that resets its strike price at certain dates.
    Cliquet {
        /// The dates (in days to expiry) on which the strike price is reset.
        reset_dates: Vec<f64>,
    },

    /// A Rainbow option based on the performance of two or more underlying assets.
    Rainbow {
        /// The number of underlying assets the option is based on.
        num_assets: usize,
        /// The type of rainbow option (BestOf or WorstOf).
        rainbow_type: RainbowType,
    },

    /// A Spread option based on the difference between the prices of two
    /// underlying assets.
    Spread {
        /// The price of the second asset involved in the spread.
        second_asset: f64,
    },

    /// A Quanto option whose payoff depends on the underlying asset price in
    /// one currency, but the payoff is made in another currency at a fixed
    /// exchange rate.
    Quanto {
        /// The fixed exchange rate at which the payoff is converted.
        exchange_rate: f64,
    },

    /// An Exchange option giving the holder the right to exchange one asset
    /// for another.
    Exchange {
        /// The price of the second asset involved in the exchange.
        second_asset: f64,
    },

    /// A Power option whose payoff is based on the underlying asset price
    /// raised to a certain power.
    Power {
        /// The exponent to which the underlying asset price is raised.
        exponent: f64,
    },
}

impl OptionType {
    /// Returns `true` if this is a [`European`](Self::European) option.
    #[must_use]
    #[inline]
    pub const fn is_european(&self) -> bool {
        matches!(self, Self::European)
    }

    /// Returns `true` if this is an [`American`](Self::American) option.
    #[must_use]
    #[inline]
    pub const fn is_american(&self) -> bool {
        matches!(self, Self::American)
    }

    /// Returns `true` if this is any exotic (non-European, non-American) option type.
    #[must_use]
    #[inline]
    pub const fn is_exotic(&self) -> bool {
        !matches!(self, Self::European | Self::American)
    }

    /// Returns `true` if this option type is path-dependent.
    ///
    /// Path-dependent options include Asian, Barrier, Lookback, and Cliquet.
    #[must_use]
    #[inline]
    pub const fn is_path_dependent(&self) -> bool {
        matches!(
            self,
            Self::Asian { .. }
                | Self::Barrier { .. }
                | Self::Lookback { .. }
                | Self::Cliquet { .. }
        )
    }

    /// Returns `true` if this option type involves multiple underlying assets.
    ///
    /// Multi-asset options include Rainbow, Spread, and Exchange.
    #[must_use]
    #[inline]
    pub const fn is_multi_asset(&self) -> bool {
        matches!(
            self,
            Self::Rainbow { .. } | Self::Spread { .. } | Self::Exchange { .. }
        )
    }
}

impl fmt::Display for OptionType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::European => write!(f, "European Option"),
            Self::American => write!(f, "American Option"),
            Self::Bermuda { exercise_dates } => {
                write!(f, "Bermuda Option (Exercise Dates: {exercise_dates:?})")
            }
            Self::Asian { averaging_type } => {
                write!(f, "Asian Option (Averaging Type: {averaging_type})")
            }
            Self::Barrier {
                barrier_type,
                barrier_level,
                rebate,
            } => {
                write!(
                    f,
                    "Barrier Option (Type: {}, Level: {}, Rebate: {:?})",
                    barrier_type, barrier_level, rebate
                )
            }
            Self::Binary { binary_type } => {
                write!(f, "Binary Option (Type: {binary_type})")
            }
            Self::Lookback { lookback_type } => {
                write!(f, "Lookback Option (Type: {lookback_type})")
            }
            Self::Compound { underlying_option } => {
                write!(f, "Compound Option (Underlying: {underlying_option})")
            }
            Self::Chooser { choice_date } => {
                write!(f, "Chooser Option (Choice Date: {choice_date})")
            }
            Self::Cliquet { reset_dates } => {
                write!(f, "Cliquet Option (Reset Dates: {reset_dates:?})")
            }
            Self::Rainbow {
                num_assets,
                rainbow_type,
            } => {
                write!(
                    f,
                    "Rainbow Option (Type: {rainbow_type:?}, Number of Assets: {num_assets})"
                )
            }
            Self::Spread { second_asset } => {
                write!(f, "Spread Option (Second Asset: {second_asset})")
            }
            Self::Quanto { exchange_rate } => {
                write!(f, "Quanto Option (Exchange Rate: {exchange_rate})")
            }
            Self::Exchange { second_asset } => {
                write!(f, "Exchange Option (Second Asset: {second_asset})")
            }
            Self::Power { exponent } => write!(f, "Power Option (Exponent: {exponent})"),
        }
    }
}

/// Describes how the average price is calculated for Asian options.
///
/// # Examples
///
/// ```rust
/// use option_type::AsianAveragingType;
///
/// let avg = AsianAveragingType::Arithmetic;
/// assert_eq!(format!("{avg}"), "Arithmetic Averaging");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[repr(u8)]
pub enum AsianAveragingType {
    /// Arithmetic averaging sums all observed prices and divides by the number
    /// of observations.
    Arithmetic = 0,
    /// Geometric averaging takes the nth root of the product of n observed prices.
    Geometric = 1,
}

impl AsianAveragingType {
    /// Returns `true` if this is [`Arithmetic`](Self::Arithmetic) averaging.
    #[must_use]
    #[inline]
    pub const fn is_arithmetic(&self) -> bool {
        matches!(self, Self::Arithmetic)
    }

    /// Returns `true` if this is [`Geometric`](Self::Geometric) averaging.
    #[must_use]
    #[inline]
    pub const fn is_geometric(&self) -> bool {
        matches!(self, Self::Geometric)
    }
}

impl fmt::Display for AsianAveragingType {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Arithmetic => write!(f, "Arithmetic Averaging"),
            Self::Geometric => write!(f, "Geometric Averaging"),
        }
    }
}

/// Describes the type of barrier for Barrier options.
///
/// Barrier options become active ("knock-in") or inactive ("knock-out") when
/// the underlying asset reaches a specified price level.
///
/// # Examples
///
/// ```rust
/// use option_type::BarrierType;
///
/// let barrier = BarrierType::UpAndIn;
/// assert!(barrier.is_knock_in());
/// assert!(!barrier.is_knock_out());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[repr(u8)]
pub enum BarrierType {
    /// The option activates if the price goes **above** a certain level.
    UpAndIn = 0,
    /// The option deactivates if the price goes **above** a certain level.
    UpAndOut = 1,
    /// The option activates if the price goes **below** a certain level.
    DownAndIn = 2,
    /// The option deactivates if the price goes **below** a certain level.
    DownAndOut = 3,
}

impl BarrierType {
    /// Returns `true` if this is a knock-in barrier (UpAndIn or DownAndIn).
    #[must_use]
    #[inline]
    pub const fn is_knock_in(&self) -> bool {
        matches!(self, Self::UpAndIn | Self::DownAndIn)
    }

    /// Returns `true` if this is a knock-out barrier (UpAndOut or DownAndOut).
    #[must_use]
    #[inline]
    pub const fn is_knock_out(&self) -> bool {
        matches!(self, Self::UpAndOut | Self::DownAndOut)
    }

    /// Returns `true` if the barrier direction is upward (UpAndIn or UpAndOut).
    #[must_use]
    #[inline]
    pub const fn is_up(&self) -> bool {
        matches!(self, Self::UpAndIn | Self::UpAndOut)
    }

    /// Returns `true` if the barrier direction is downward (DownAndIn or DownAndOut).
    #[must_use]
    #[inline]
    pub const fn is_down(&self) -> bool {
        matches!(self, Self::DownAndIn | Self::DownAndOut)
    }
}

impl fmt::Display for BarrierType {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::UpAndIn => write!(f, "Up-And-In Barrier"),
            Self::UpAndOut => write!(f, "Up-And-Out Barrier"),
            Self::DownAndIn => write!(f, "Down-And-In Barrier"),
            Self::DownAndOut => write!(f, "Down-And-Out Barrier"),
        }
    }
}

/// Represents different types of binary options, which are financial instruments
/// that provide a fixed payout based on whether certain conditions are met.
///
/// # Examples
///
/// ```rust
/// use option_type::BinaryType;
///
/// let binary = BinaryType::CashOrNothing;
/// assert_eq!(format!("{binary}"), "Cash-Or-Nothing Binary Option");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[repr(u8)]
pub enum BinaryType {
    /// The option pays a fixed amount of cash if the underlying asset is above
    /// or below a certain level.
    CashOrNothing = 0,
    /// The option pays the value of the underlying asset if it is above or below
    /// a certain level.
    AssetOrNothing = 1,
    /// Pays out if the underlying asset price is above the strike price at
    /// expiration, with the payout proportional to how far above the strike it is.
    Gap = 2,
}

impl fmt::Display for BinaryType {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::CashOrNothing => write!(f, "Cash-Or-Nothing Binary Option"),
            Self::AssetOrNothing => write!(f, "Asset-Or-Nothing Binary Option"),
            Self::Gap => write!(f, "Gap Binary Option"),
        }
    }
}

/// Describes the type of lookback option.
///
/// Lookback options allow the holder to determine the payoff based on the
/// historical maximum or minimum price of the underlying asset.
///
/// # Examples
///
/// ```rust
/// use option_type::LookbackType;
///
/// let lookback = LookbackType::FixedStrike;
/// assert_eq!(format!("{lookback}"), "Fixed-Strike Lookback Option");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[repr(u8)]
pub enum LookbackType {
    /// The strike price is fixed at the beginning, and the payoff is based on
    /// the maximum or minimum price of the underlying asset during the option's life.
    FixedStrike = 0,
    /// The strike price is determined as the maximum or minimum price of the
    /// underlying asset during the option's life, providing the holder with the
    /// most advantageous strike price.
    FloatingStrike = 1,
}

impl fmt::Display for LookbackType {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::FixedStrike => write!(f, "Fixed-Strike Lookback Option"),
            Self::FloatingStrike => write!(f, "Floating-Strike Lookback Option"),
        }
    }
}

/// Describes the type of rainbow option based on how multiple assets are combined.
///
/// Rainbow options are multi-asset options where the payoff depends on the
/// relative performance of two or more underlying assets.
///
/// # Examples
///
/// ```rust
/// use option_type::RainbowType;
///
/// let rainbow = RainbowType::default();
/// assert_eq!(rainbow, RainbowType::BestOf);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[repr(u8)]
pub enum RainbowType {
    /// Option on the maximum of multiple assets.
    /// Call payoff: max(max(S1, S2, ..., Sn) - K, 0)
    /// Put payoff: max(K - max(S1, S2, ..., Sn), 0)
    #[default]
    BestOf = 0,
    /// Option on the minimum of multiple assets.
    /// Call payoff: max(min(S1, S2, ..., Sn) - K, 0)
    /// Put payoff: max(K - min(S1, S2, ..., Sn), 0)
    WorstOf = 1,
}

impl RainbowType {
    /// Returns `true` if this is a [`BestOf`](Self::BestOf) rainbow option.
    #[must_use]
    #[inline]
    pub const fn is_best_of(&self) -> bool {
        matches!(self, Self::BestOf)
    }

    /// Returns `true` if this is a [`WorstOf`](Self::WorstOf) rainbow option.
    #[must_use]
    #[inline]
    pub const fn is_worst_of(&self) -> bool {
        matches!(self, Self::WorstOf)
    }
}

/// A structure representing the basic properties of an option in financial terms.
///
/// This structure is designed to be lightweight and provides essential details
/// about an options contract through borrowed references.
///
/// # Examples
///
/// ```rust
/// use option_type::OptionBasicType;
/// use financial_types::{OptionStyle, Side};
/// use positive::Positive;
/// use expiration_date::ExpirationDate;
/// use positive::pos_or_panic;
///
/// let european_call_option = OptionBasicType {
///     option_style: &OptionStyle::Call,
///     side: &Side::Long,
///     strike_price: &Positive::new(100.0).unwrap(),
///     expiration_date: &ExpirationDate::Days(pos_or_panic!(30.0)),
/// };
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct OptionBasicType<'a> {
    /// The style of the option (Call or Put).
    pub option_style: &'a OptionStyle,
    /// The side of the position (Long or Short).
    pub side: &'a Side,
    /// The strike price of the option, guaranteed to be positive.
    pub strike_price: &'a Positive,
    /// The expiration date of the option.
    pub expiration_date: &'a ExpirationDate,
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic, clippy::expect_used)]
mod tests_option_type {
    use super::*;

    #[test]
    fn test_default() {
        assert_eq!(OptionType::default(), OptionType::European);
    }

    #[test]
    fn test_is_european() {
        assert!(OptionType::European.is_european());
        assert!(!OptionType::American.is_european());
        assert!(!OptionType::Power { exponent: 2.0 }.is_european());
    }

    #[test]
    fn test_is_american() {
        assert!(OptionType::American.is_american());
        assert!(!OptionType::European.is_american());
    }

    #[test]
    fn test_is_exotic() {
        assert!(!OptionType::European.is_exotic());
        assert!(!OptionType::American.is_exotic());
        assert!(
            OptionType::Asian {
                averaging_type: AsianAveragingType::Arithmetic
            }
            .is_exotic()
        );
        assert!(OptionType::Power { exponent: 2.0 }.is_exotic());
        assert!(
            OptionType::Bermuda {
                exercise_dates: vec![30.0]
            }
            .is_exotic()
        );
    }

    #[test]
    fn test_is_path_dependent() {
        assert!(
            OptionType::Asian {
                averaging_type: AsianAveragingType::Arithmetic
            }
            .is_path_dependent()
        );
        assert!(
            OptionType::Barrier {
                barrier_type: BarrierType::UpAndIn,
                barrier_level: 120.0,
                rebate: None,
            }
            .is_path_dependent()
        );
        assert!(
            OptionType::Lookback {
                lookback_type: LookbackType::FixedStrike
            }
            .is_path_dependent()
        );
        assert!(
            OptionType::Cliquet {
                reset_dates: vec![30.0]
            }
            .is_path_dependent()
        );
        assert!(!OptionType::European.is_path_dependent());
        assert!(!OptionType::Power { exponent: 2.0 }.is_path_dependent());
    }

    #[test]
    fn test_is_multi_asset() {
        assert!(
            OptionType::Rainbow {
                num_assets: 2,
                rainbow_type: RainbowType::BestOf
            }
            .is_multi_asset()
        );
        assert!(OptionType::Spread { second_asset: 90.0 }.is_multi_asset());
        assert!(
            OptionType::Exchange {
                second_asset: 110.0
            }
            .is_multi_asset()
        );
        assert!(!OptionType::European.is_multi_asset());
        assert!(!OptionType::Quanto { exchange_rate: 1.5 }.is_multi_asset());
    }

    #[test]
    fn test_display_european() {
        assert_eq!(format!("{}", OptionType::European), "European Option");
    }

    #[test]
    fn test_display_american() {
        assert_eq!(format!("{}", OptionType::American), "American Option");
    }

    #[test]
    fn test_display_bermuda() {
        let opt = OptionType::Bermuda {
            exercise_dates: vec![30.0, 60.0, 90.0],
        };
        assert!(format!("{opt}").contains("Bermuda"));
    }

    #[test]
    fn test_display_asian() {
        let opt = OptionType::Asian {
            averaging_type: AsianAveragingType::Arithmetic,
        };
        assert!(format!("{opt}").contains("Asian"));
        assert!(format!("{opt}").contains("Arithmetic"));
    }

    #[test]
    fn test_display_barrier() {
        let opt = OptionType::Barrier {
            barrier_type: BarrierType::UpAndIn,
            barrier_level: 120.0,
            rebate: Some(5.0),
        };
        let display = format!("{opt}");
        assert!(display.contains("Barrier"));
        assert!(display.contains("120"));
    }

    #[test]
    fn test_display_binary() {
        let opt = OptionType::Binary {
            binary_type: BinaryType::CashOrNothing,
        };
        assert!(format!("{opt}").contains("Binary"));
    }

    #[test]
    fn test_display_lookback() {
        let opt = OptionType::Lookback {
            lookback_type: LookbackType::FixedStrike,
        };
        assert!(format!("{opt}").contains("Lookback"));
    }

    #[test]
    fn test_display_compound() {
        let opt = OptionType::Compound {
            underlying_option: Box::new(OptionType::European),
        };
        assert!(format!("{opt}").contains("Compound"));
    }

    #[test]
    fn test_display_chooser() {
        let opt = OptionType::Chooser { choice_date: 30.0 };
        assert!(format!("{opt}").contains("Chooser"));
    }

    #[test]
    fn test_display_cliquet() {
        let opt = OptionType::Cliquet {
            reset_dates: vec![30.0, 60.0],
        };
        assert!(format!("{opt}").contains("Cliquet"));
    }

    #[test]
    fn test_display_rainbow() {
        let opt = OptionType::Rainbow {
            num_assets: 3,
            rainbow_type: RainbowType::BestOf,
        };
        assert!(format!("{opt}").contains("Rainbow"));
    }

    #[test]
    fn test_display_spread() {
        let opt = OptionType::Spread { second_asset: 90.0 };
        assert!(format!("{opt}").contains("Spread"));
    }

    #[test]
    fn test_display_quanto() {
        let opt = OptionType::Quanto { exchange_rate: 1.5 };
        assert!(format!("{opt}").contains("Quanto"));
    }

    #[test]
    fn test_display_exchange() {
        let opt = OptionType::Exchange {
            second_asset: 110.0,
        };
        assert!(format!("{opt}").contains("Exchange"));
    }

    #[test]
    fn test_display_power() {
        let opt = OptionType::Power { exponent: 2.0 };
        assert!(format!("{opt}").contains("Power"));
    }

    #[test]
    fn test_clone() {
        let opt = OptionType::Asian {
            averaging_type: AsianAveragingType::Geometric,
        };
        let cloned = opt.clone();
        assert_eq!(opt, cloned);
    }

    #[test]
    fn test_serialization_european() {
        let opt = OptionType::European;
        let json = serde_json::to_string(&opt).unwrap();
        let deserialized: OptionType = serde_json::from_str(&json).unwrap();
        assert_eq!(opt, deserialized);
    }

    #[test]
    fn test_serialization_asian() {
        let opt = OptionType::Asian {
            averaging_type: AsianAveragingType::Arithmetic,
        };
        let json = serde_json::to_string(&opt).unwrap();
        let deserialized: OptionType = serde_json::from_str(&json).unwrap();
        assert_eq!(opt, deserialized);
    }

    #[test]
    fn test_serialization_barrier() {
        let opt = OptionType::Barrier {
            barrier_type: BarrierType::DownAndOut,
            barrier_level: 90.0,
            rebate: None,
        };
        let json = serde_json::to_string(&opt).unwrap();
        let deserialized: OptionType = serde_json::from_str(&json).unwrap();
        assert_eq!(opt, deserialized);
    }

    #[test]
    fn test_serialization_rainbow() {
        let opt = OptionType::Rainbow {
            num_assets: 2,
            rainbow_type: RainbowType::WorstOf,
        };
        let json = serde_json::to_string(&opt).unwrap();
        let deserialized: OptionType = serde_json::from_str(&json).unwrap();
        assert_eq!(opt, deserialized);
    }

    #[test]
    fn test_compound_skip_serialization() {
        let opt = OptionType::Compound {
            underlying_option: Box::new(OptionType::European),
        };
        let json = serde_json::to_string(&opt);
        assert!(
            json.is_err() || {
                let deserialized: OptionType = serde_json::from_str(&json.unwrap()).unwrap();
                deserialized == OptionType::European
            }
        );
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic, clippy::expect_used)]
mod tests_sub_enums {
    use super::*;

    #[test]
    fn test_asian_averaging_type_display() {
        assert_eq!(
            format!("{}", AsianAveragingType::Arithmetic),
            "Arithmetic Averaging"
        );
        assert_eq!(
            format!("{}", AsianAveragingType::Geometric),
            "Geometric Averaging"
        );
    }

    #[test]
    fn test_asian_averaging_type_helpers() {
        assert!(AsianAveragingType::Arithmetic.is_arithmetic());
        assert!(!AsianAveragingType::Arithmetic.is_geometric());
        assert!(AsianAveragingType::Geometric.is_geometric());
        assert!(!AsianAveragingType::Geometric.is_arithmetic());
    }

    #[test]
    fn test_barrier_type_display() {
        assert_eq!(format!("{}", BarrierType::UpAndIn), "Up-And-In Barrier");
        assert_eq!(format!("{}", BarrierType::UpAndOut), "Up-And-Out Barrier");
        assert_eq!(format!("{}", BarrierType::DownAndIn), "Down-And-In Barrier");
        assert_eq!(
            format!("{}", BarrierType::DownAndOut),
            "Down-And-Out Barrier"
        );
    }

    #[test]
    fn test_barrier_type_helpers() {
        assert!(BarrierType::UpAndIn.is_knock_in());
        assert!(BarrierType::DownAndIn.is_knock_in());
        assert!(!BarrierType::UpAndOut.is_knock_in());
        assert!(!BarrierType::DownAndOut.is_knock_in());

        assert!(BarrierType::UpAndOut.is_knock_out());
        assert!(BarrierType::DownAndOut.is_knock_out());
        assert!(!BarrierType::UpAndIn.is_knock_out());
        assert!(!BarrierType::DownAndIn.is_knock_out());

        assert!(BarrierType::UpAndIn.is_up());
        assert!(BarrierType::UpAndOut.is_up());
        assert!(!BarrierType::DownAndIn.is_up());
        assert!(!BarrierType::DownAndOut.is_up());

        assert!(BarrierType::DownAndIn.is_down());
        assert!(BarrierType::DownAndOut.is_down());
        assert!(!BarrierType::UpAndIn.is_down());
        assert!(!BarrierType::UpAndOut.is_down());
    }

    #[test]
    fn test_binary_type_display() {
        assert_eq!(
            format!("{}", BinaryType::CashOrNothing),
            "Cash-Or-Nothing Binary Option"
        );
        assert_eq!(
            format!("{}", BinaryType::AssetOrNothing),
            "Asset-Or-Nothing Binary Option"
        );
        assert_eq!(format!("{}", BinaryType::Gap), "Gap Binary Option");
    }

    #[test]
    fn test_lookback_type_display() {
        assert_eq!(
            format!("{}", LookbackType::FixedStrike),
            "Fixed-Strike Lookback Option"
        );
        assert_eq!(
            format!("{}", LookbackType::FloatingStrike),
            "Floating-Strike Lookback Option"
        );
    }

    #[test]
    fn test_rainbow_type_default() {
        assert_eq!(RainbowType::default(), RainbowType::BestOf);
    }

    #[test]
    fn test_rainbow_type_helpers() {
        assert!(RainbowType::BestOf.is_best_of());
        assert!(!RainbowType::BestOf.is_worst_of());
        assert!(RainbowType::WorstOf.is_worst_of());
        assert!(!RainbowType::WorstOf.is_best_of());
    }

    #[test]
    fn test_sub_enum_serialization_roundtrip() {
        let asian = AsianAveragingType::Geometric;
        let json = serde_json::to_string(&asian).unwrap();
        let deserialized: AsianAveragingType = serde_json::from_str(&json).unwrap();
        assert_eq!(asian, deserialized);

        let barrier = BarrierType::DownAndIn;
        let json = serde_json::to_string(&barrier).unwrap();
        let deserialized: BarrierType = serde_json::from_str(&json).unwrap();
        assert_eq!(barrier, deserialized);

        let binary = BinaryType::Gap;
        let json = serde_json::to_string(&binary).unwrap();
        let deserialized: BinaryType = serde_json::from_str(&json).unwrap();
        assert_eq!(binary, deserialized);

        let lookback = LookbackType::FloatingStrike;
        let json = serde_json::to_string(&lookback).unwrap();
        let deserialized: LookbackType = serde_json::from_str(&json).unwrap();
        assert_eq!(lookback, deserialized);

        let rainbow = RainbowType::WorstOf;
        let json = serde_json::to_string(&rainbow).unwrap();
        let deserialized: RainbowType = serde_json::from_str(&json).unwrap();
        assert_eq!(rainbow, deserialized);
    }

    #[test]
    fn test_repr_u8_sizes() {
        assert_eq!(
            std::mem::size_of::<AsianAveragingType>(),
            1,
            "AsianAveragingType should be 1 byte with #[repr(u8)]"
        );
        assert_eq!(
            std::mem::size_of::<BarrierType>(),
            1,
            "BarrierType should be 1 byte with #[repr(u8)]"
        );
        assert_eq!(
            std::mem::size_of::<BinaryType>(),
            1,
            "BinaryType should be 1 byte with #[repr(u8)]"
        );
        assert_eq!(
            std::mem::size_of::<LookbackType>(),
            1,
            "LookbackType should be 1 byte with #[repr(u8)]"
        );
        assert_eq!(
            std::mem::size_of::<RainbowType>(),
            1,
            "RainbowType should be 1 byte with #[repr(u8)]"
        );
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic, clippy::expect_used)]
mod tests_option_basic_type {
    use super::*;
    use positive::pos_or_panic;

    #[test]
    fn test_creation() {
        let style = OptionStyle::Call;
        let side = Side::Long;
        let strike = pos_or_panic!(100.0);
        let expiration = ExpirationDate::Days(pos_or_panic!(30.0));

        let basic = OptionBasicType {
            option_style: &style,
            side: &side,
            strike_price: &strike,
            expiration_date: &expiration,
        };

        assert_eq!(*basic.option_style, OptionStyle::Call);
        assert_eq!(*basic.side, Side::Long);
        assert_eq!(*basic.strike_price, strike);
    }

    #[test]
    fn test_equality() {
        let style = OptionStyle::Put;
        let side = Side::Short;
        let strike = pos_or_panic!(200.0);
        let expiration = ExpirationDate::Days(pos_or_panic!(60.0));

        let basic1 = OptionBasicType {
            option_style: &style,
            side: &side,
            strike_price: &strike,
            expiration_date: &expiration,
        };

        let basic2 = OptionBasicType {
            option_style: &style,
            side: &side,
            strike_price: &strike,
            expiration_date: &expiration,
        };

        assert_eq!(basic1, basic2);
    }

    #[test]
    fn test_copy() {
        let style = OptionStyle::Call;
        let side = Side::Long;
        let strike = pos_or_panic!(150.0);
        let expiration = ExpirationDate::Days(pos_or_panic!(45.0));

        let basic = OptionBasicType {
            option_style: &style,
            side: &side,
            strike_price: &strike,
            expiration_date: &expiration,
        };

        let copied = basic;
        assert_eq!(basic, copied);
    }

    #[test]
    fn test_hash() {
        use std::collections::HashSet;

        let style = OptionStyle::Call;
        let side = Side::Long;
        let strike = pos_or_panic!(100.0);
        let expiration = ExpirationDate::Days(pos_or_panic!(30.0));

        let basic = OptionBasicType {
            option_style: &style,
            side: &side,
            strike_price: &strike,
            expiration_date: &expiration,
        };

        let mut set = HashSet::new();
        set.insert(basic);
        set.insert(basic); // duplicate
        assert_eq!(set.len(), 1);
    }
}
