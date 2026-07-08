//! Leaf sub-enums used within exotic [`OptionType`](crate::OptionType) variants.
//!
//! Each sub-enum is a compact, discriminant-only `#[repr(u8)]` enum and lives
//! in its own module:
//!
//! - [`AsianAveragingType`] — Arithmetic or Geometric averaging
//! - [`BarrierType`] — Up/Down and In/Out barrier conditions
//! - [`BinaryType`] — Cash-or-nothing, Asset-or-nothing, Gap
//! - [`LookbackType`] — Fixed or Floating strike
//! - [`RainbowType`] — Best-of or Worst-of multi-asset

pub mod asian;
pub mod barrier;
pub mod binary;
pub mod lookback;
pub mod rainbow;

pub use asian::AsianAveragingType;
pub use barrier::BarrierType;
pub use binary::BinaryType;
pub use lookback::LookbackType;
pub use rainbow::RainbowType;

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic, clippy::expect_used)]
mod tests {
    use super::*;

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
