//! The [`OptionType`] enum, its classification helpers, and its `Display` impl.

use crate::sub_enums::{AsianAveragingType, BarrierType, BinaryType, LookbackType, RainbowType};
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

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic, clippy::expect_used)]
mod tests {
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
