//! Lookback option strike type ([`LookbackType`]).

use serde::{Deserialize, Serialize};
use std::fmt;

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

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic, clippy::expect_used)]
mod tests {
    use super::*;

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
}
