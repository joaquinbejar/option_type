//! Asian option averaging type ([`AsianAveragingType`]).

use serde::{Deserialize, Serialize};
use std::fmt;

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

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic, clippy::expect_used)]
mod tests {
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
}
