//! Rainbow (multi-asset) option type ([`RainbowType`]).

use serde::{Deserialize, Serialize};

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

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic, clippy::expect_used)]
mod tests {
    use super::*;

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
}
