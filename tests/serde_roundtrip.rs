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
use positive::pos_or_panic;
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
        exercise_dates: vec![
            pos_or_panic!(30.0),
            pos_or_panic!(60.0),
            pos_or_panic!(90.0),
        ],
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
        barrier_level: pos_or_panic!(120.0),
        rebate: Some(pos_or_panic!(5.0)),
    });
    assert_roundtrip(&OptionType::Barrier {
        barrier_type: BarrierType::DownAndOut,
        barrier_level: pos_or_panic!(90.0),
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
    assert_roundtrip(&OptionType::Chooser {
        choice_date: pos_or_panic!(30.0),
    });
}

#[test]
fn test_serde_option_type_cliquet_roundtrip_equals_original() {
    assert_roundtrip(&OptionType::Cliquet {
        reset_dates: vec![pos_or_panic!(30.0), pos_or_panic!(60.0)],
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
    assert_roundtrip(&OptionType::Spread {
        second_asset: pos_or_panic!(90.0),
    });
}

#[test]
fn test_serde_option_type_exchange_roundtrip_equals_original() {
    assert_roundtrip(&OptionType::Exchange {
        second_asset: pos_or_panic!(110.0),
    });
}

#[test]
fn test_serde_option_type_quanto_roundtrip_equals_original() {
    assert_roundtrip(&OptionType::Quanto {
        exchange_rate: pos_or_panic!(1.5),
    });
}

#[test]
fn test_serde_option_type_power_roundtrip_equals_original() {
    assert_roundtrip(&OptionType::Power {
        exponent: pos_or_panic!(2.0),
    });
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
// Negative-rejection on deserialization
//
// `Positive` payload fields serialize to a plain JSON number, but a negative
// number in the wire form must be rejected on deserialization. These tests
// serialize a valid value, confirm the positive form round-trips, then flip the
// numeric token negative and assert the deserialization fails.
// ---------------------------------------------------------------------------

#[test]
fn test_serde_barrier_negative_barrier_level_returns_error() {
    let valid = OptionType::Barrier {
        barrier_type: BarrierType::UpAndIn,
        barrier_level: pos_or_panic!(120.0),
        rebate: None,
    };
    let json = serde_json::to_string(&valid).expect("valid barrier serializes");
    // Sanity: the positive wire form deserializes back.
    assert!(
        serde_json::from_str::<OptionType>(&json).is_ok(),
        "positive barrier_level must deserialize, json: {json}"
    );

    // Flip exactly `Barrier.barrier_level` negative in the parsed wire form.
    let mut value: serde_json::Value = serde_json::from_str(&json).expect("wire form parses");
    value["Barrier"]["barrier_level"] = serde_json::json!(-120.0);
    let negative_json = serde_json::to_string(&value).expect("mutated value serializes");
    let result = serde_json::from_str::<OptionType>(&negative_json);
    assert!(
        result.is_err(),
        "negative barrier_level must be rejected on deserialization, got: {result:?}"
    );
}

#[test]
fn test_serde_bermuda_negative_exercise_date_returns_error() {
    let valid = OptionType::Bermuda {
        exercise_dates: vec![pos_or_panic!(30.0)],
    };
    let json = serde_json::to_string(&valid).expect("valid bermuda serializes");
    // Sanity: the positive wire form deserializes back.
    assert!(
        serde_json::from_str::<OptionType>(&json).is_ok(),
        "positive exercise_dates must deserialize, json: {json}"
    );

    // Flip exactly `Bermuda.exercise_dates[0]` negative in the parsed wire form.
    let mut value: serde_json::Value = serde_json::from_str(&json).expect("wire form parses");
    value["Bermuda"]["exercise_dates"][0] = serde_json::json!(-30.0);
    let negative_json = serde_json::to_string(&value).expect("mutated value serializes");
    let result = serde_json::from_str::<OptionType>(&negative_json);
    assert!(
        result.is_err(),
        "negative exercise_date in the Vec must be rejected on deserialization, got: {result:?}"
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
