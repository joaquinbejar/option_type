//! Integration tests: `Display` output is stable and matches the documented
//! human-readable labels exactly.
//!
//! Downstream code may match on these strings, so every label is asserted with
//! exact equality. The expected strings are copied verbatim from the `Display`
//! impls in `src/option_type.rs` and `src/sub_enums/*.rs`.
//!
//! Note: [`RainbowType`] has no `Display` impl; the `Rainbow` variant renders
//! its sub-type via `Debug`, which is covered by the `OptionType` cases below.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use option_type::prelude::*;

// ---------------------------------------------------------------------------
// OptionType
// ---------------------------------------------------------------------------

#[test]
fn test_display_option_type_european_matches_label() {
    assert_eq!(format!("{}", OptionType::European), "European Option");
}

#[test]
fn test_display_option_type_american_matches_label() {
    assert_eq!(format!("{}", OptionType::American), "American Option");
}

#[test]
fn test_display_option_type_bermuda_matches_label() {
    let opt = OptionType::Bermuda {
        exercise_dates: vec![30.0, 60.0, 90.0],
    };
    assert_eq!(
        format!("{opt}"),
        "Bermuda Option (Exercise Dates: [30.0, 60.0, 90.0])"
    );
}

#[test]
fn test_display_option_type_asian_matches_label() {
    let arithmetic = OptionType::Asian {
        averaging_type: AsianAveragingType::Arithmetic,
    };
    assert_eq!(
        format!("{arithmetic}"),
        "Asian Option (Averaging Type: Arithmetic Averaging)"
    );

    let geometric = OptionType::Asian {
        averaging_type: AsianAveragingType::Geometric,
    };
    assert_eq!(
        format!("{geometric}"),
        "Asian Option (Averaging Type: Geometric Averaging)"
    );
}

#[test]
fn test_display_option_type_barrier_matches_label() {
    let with_rebate = OptionType::Barrier {
        barrier_type: BarrierType::UpAndIn,
        barrier_level: 120.0,
        rebate: Some(5.0),
    };
    assert_eq!(
        format!("{with_rebate}"),
        "Barrier Option (Type: Up-And-In Barrier, Level: 120, Rebate: Some(5.0))"
    );

    let without_rebate = OptionType::Barrier {
        barrier_type: BarrierType::DownAndOut,
        barrier_level: 90.0,
        rebate: None,
    };
    assert_eq!(
        format!("{without_rebate}"),
        "Barrier Option (Type: Down-And-Out Barrier, Level: 90, Rebate: None)"
    );
}

#[test]
fn test_display_option_type_binary_matches_label() {
    let opt = OptionType::Binary {
        binary_type: BinaryType::CashOrNothing,
    };
    assert_eq!(
        format!("{opt}"),
        "Binary Option (Type: Cash-Or-Nothing Binary Option)"
    );
}

#[test]
fn test_display_option_type_lookback_matches_label() {
    let opt = OptionType::Lookback {
        lookback_type: LookbackType::FixedStrike,
    };
    assert_eq!(
        format!("{opt}"),
        "Lookback Option (Type: Fixed-Strike Lookback Option)"
    );
}

#[test]
fn test_display_option_type_compound_matches_label() {
    let opt = OptionType::Compound {
        underlying_option: Box::new(OptionType::European),
    };
    assert_eq!(
        format!("{opt}"),
        "Compound Option (Underlying: European Option)"
    );
}

#[test]
fn test_display_option_type_chooser_matches_label() {
    let opt = OptionType::Chooser { choice_date: 30.0 };
    assert_eq!(format!("{opt}"), "Chooser Option (Choice Date: 30)");
}

#[test]
fn test_display_option_type_cliquet_matches_label() {
    let opt = OptionType::Cliquet {
        reset_dates: vec![30.0, 60.0],
    };
    assert_eq!(
        format!("{opt}"),
        "Cliquet Option (Reset Dates: [30.0, 60.0])"
    );
}

#[test]
fn test_display_option_type_rainbow_matches_label() {
    let best_of = OptionType::Rainbow {
        num_assets: 3,
        rainbow_type: RainbowType::BestOf,
    };
    assert_eq!(
        format!("{best_of}"),
        "Rainbow Option (Type: BestOf, Number of Assets: 3)"
    );

    let worst_of = OptionType::Rainbow {
        num_assets: 2,
        rainbow_type: RainbowType::WorstOf,
    };
    assert_eq!(
        format!("{worst_of}"),
        "Rainbow Option (Type: WorstOf, Number of Assets: 2)"
    );
}

#[test]
fn test_display_option_type_spread_matches_label() {
    let opt = OptionType::Spread { second_asset: 90.0 };
    assert_eq!(format!("{opt}"), "Spread Option (Second Asset: 90)");
}

#[test]
fn test_display_option_type_quanto_matches_label() {
    let opt = OptionType::Quanto { exchange_rate: 1.5 };
    assert_eq!(format!("{opt}"), "Quanto Option (Exchange Rate: 1.5)");
}

#[test]
fn test_display_option_type_exchange_matches_label() {
    let opt = OptionType::Exchange {
        second_asset: 110.0,
    };
    assert_eq!(format!("{opt}"), "Exchange Option (Second Asset: 110)");
}

#[test]
fn test_display_option_type_power_matches_label() {
    let opt = OptionType::Power { exponent: 2.0 };
    assert_eq!(format!("{opt}"), "Power Option (Exponent: 2)");
}

// ---------------------------------------------------------------------------
// AsianAveragingType
// ---------------------------------------------------------------------------

#[test]
fn test_display_asian_averaging_type_all_variants_match_labels() {
    assert_eq!(
        format!("{}", AsianAveragingType::Arithmetic),
        "Arithmetic Averaging"
    );
    assert_eq!(
        format!("{}", AsianAveragingType::Geometric),
        "Geometric Averaging"
    );
}

// ---------------------------------------------------------------------------
// BarrierType
// ---------------------------------------------------------------------------

#[test]
fn test_display_barrier_type_all_variants_match_labels() {
    assert_eq!(format!("{}", BarrierType::UpAndIn), "Up-And-In Barrier");
    assert_eq!(format!("{}", BarrierType::UpAndOut), "Up-And-Out Barrier");
    assert_eq!(format!("{}", BarrierType::DownAndIn), "Down-And-In Barrier");
    assert_eq!(
        format!("{}", BarrierType::DownAndOut),
        "Down-And-Out Barrier"
    );
}

// ---------------------------------------------------------------------------
// BinaryType
// ---------------------------------------------------------------------------

#[test]
fn test_display_binary_type_all_variants_match_labels() {
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

// ---------------------------------------------------------------------------
// LookbackType
// ---------------------------------------------------------------------------

#[test]
fn test_display_lookback_type_all_variants_match_labels() {
    assert_eq!(
        format!("{}", LookbackType::FixedStrike),
        "Fixed-Strike Lookback Option"
    );
    assert_eq!(
        format!("{}", LookbackType::FloatingStrike),
        "Floating-Strike Lookback Option"
    );
}
