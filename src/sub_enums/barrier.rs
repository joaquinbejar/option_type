//! Barrier option barrier type ([`BarrierType`]).

use serde::{Deserialize, Serialize};
use std::fmt;

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
#[non_exhaustive]
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

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic, clippy::expect_used)]
mod tests {
    use super::*;

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
}
