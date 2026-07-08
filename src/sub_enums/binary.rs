//! Binary option payoff type ([`BinaryType`]).

use serde::{Deserialize, Serialize};
use std::fmt;

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

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic, clippy::expect_used)]
mod tests {
    use super::*;

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
}
