//! Integration tests: `serde_json` round-trip for every variant of every
//! public enum, exercised through the crate's public surface only.
//!
//! A downstream user relies on being able to serialize an [`OptionType`] (and
//! its leaf sub-enums) to JSON and read it back unchanged. Every variant that
//! is serializable is round-tripped and asserted equal to the original.
//!
//! Note: [`OptionType::Compound`] is annotated `#[serde(skip)]` in the source,
//! so it cannot be serialized. That behavior is asserted explicitly instead of
//! round-tripped.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use option_type::prelude::*;
use serde::Serialize;
use serde::de::DeserializeOwned;

/// Serialize `value` to JSON, deserialize it back, and assert equality.
fn assert_roundtrip<T>(value: &T)
where
    T: Serialize + DeserializeOwned + PartialEq + std::fmt::Debug,
{
    let json = serde_json::to_string(value).expect("value serializes to JSON");
    let back: T = serde_json::from_str(&json).expect("JSON deserializes back to value");
    assert_eq!(*value, back, "round-trip must preserve the original value");
}

// ---------------------------------------------------------------------------
// OptionType variants
// ---------------------------------------------------------------------------

#[test]
fn test_serde_option_type_european_roundtrip_equals_original() {
    assert_roundtrip(&OptionType::European);
}

#[test]
fn test_serde_option_type_american_roundtrip_equals_original() {
    assert_roundtrip(&OptionType::American);
}

#[test]
fn test_serde_option_type_bermuda_roundtrip_equals_original() {
    assert_roundtrip(&OptionType::Bermuda {
        exercise_dates: vec![30.0, 60.0, 90.0],
    });
}

#[test]
fn test_serde_option_type_asian_roundtrip_equals_original() {
    assert_roundtrip(&OptionType::Asian {
        averaging_type: AsianAveragingType::Arithmetic,
    });
    assert_roundtrip(&OptionType::Asian {
        averaging_type: AsianAveragingType::Geometric,
    });
}

#[test]
fn test_serde_option_type_barrier_roundtrip_equals_original() {
    assert_roundtrip(&OptionType::Barrier {
        barrier_type: BarrierType::UpAndIn,
        barrier_level: 120.0,
        rebate: Some(5.0),
    });
    assert_roundtrip(&OptionType::Barrier {
        barrier_type: BarrierType::DownAndOut,
        barrier_level: 90.0,
        rebate: None,
    });
}

#[test]
fn test_serde_option_type_binary_roundtrip_equals_original() {
    assert_roundtrip(&OptionType::Binary {
        binary_type: BinaryType::CashOrNothing,
    });
}

#[test]
fn test_serde_option_type_lookback_roundtrip_equals_original() {
    assert_roundtrip(&OptionType::Lookback {
        lookback_type: LookbackType::FixedStrike,
    });
}

#[test]
fn test_serde_option_type_chooser_roundtrip_equals_original() {
    assert_roundtrip(&OptionType::Chooser { choice_date: 30.0 });
}

#[test]
fn test_serde_option_type_cliquet_roundtrip_equals_original() {
    assert_roundtrip(&OptionType::Cliquet {
        reset_dates: vec![30.0, 60.0],
    });
}

#[test]
fn test_serde_option_type_rainbow_roundtrip_equals_original() {
    assert_roundtrip(&OptionType::Rainbow {
        num_assets: 3,
        rainbow_type: RainbowType::BestOf,
    });
    assert_roundtrip(&OptionType::Rainbow {
        num_assets: 2,
        rainbow_type: RainbowType::WorstOf,
    });
}

#[test]
fn test_serde_option_type_spread_roundtrip_equals_original() {
    assert_roundtrip(&OptionType::Spread { second_asset: 90.0 });
}

#[test]
fn test_serde_option_type_exchange_roundtrip_equals_original() {
    assert_roundtrip(&OptionType::Exchange {
        second_asset: 110.0,
    });
}

#[test]
fn test_serde_option_type_quanto_roundtrip_equals_original() {
    assert_roundtrip(&OptionType::Quanto { exchange_rate: 1.5 });
}

#[test]
fn test_serde_option_type_power_roundtrip_equals_original() {
    assert_roundtrip(&OptionType::Power { exponent: 2.0 });
}

#[test]
fn test_serde_option_type_compound_serialize_returns_error() {
    // `Compound` carries `#[serde(skip)]` in the source, so it is deliberately
    // not serializable. Confirm the public surface reflects that contract.
    let compound = OptionType::Compound {
        underlying_option: Box::new(OptionType::European),
    };
    let result = serde_json::to_string(&compound);
    assert!(
        result.is_err(),
        "Compound is #[serde(skip)] and must fail to serialize, got: {result:?}"
    );
}

// ---------------------------------------------------------------------------
// Leaf sub-enum variants
// ---------------------------------------------------------------------------

#[test]
fn test_serde_asian_averaging_type_all_variants_roundtrip_equals_original() {
    for value in [
        AsianAveragingType::Arithmetic,
        AsianAveragingType::Geometric,
    ] {
        assert_roundtrip(&value);
    }
}

#[test]
fn test_serde_barrier_type_all_variants_roundtrip_equals_original() {
    for value in [
        BarrierType::UpAndIn,
        BarrierType::UpAndOut,
        BarrierType::DownAndIn,
        BarrierType::DownAndOut,
    ] {
        assert_roundtrip(&value);
    }
}

#[test]
fn test_serde_binary_type_all_variants_roundtrip_equals_original() {
    for value in [
        BinaryType::CashOrNothing,
        BinaryType::AssetOrNothing,
        BinaryType::Gap,
    ] {
        assert_roundtrip(&value);
    }
}

#[test]
fn test_serde_lookback_type_all_variants_roundtrip_equals_original() {
    for value in [LookbackType::FixedStrike, LookbackType::FloatingStrike] {
        assert_roundtrip(&value);
    }
}

#[test]
fn test_serde_rainbow_type_all_variants_roundtrip_equals_original() {
    for value in [RainbowType::BestOf, RainbowType::WorstOf] {
        assert_roundtrip(&value);
    }
}
