//! Integration tests: every `is_*` classification helper is exercised on every
//! variant of its enum, with expectations derived from the source truth table.
//!
//! Helpers under test (from `src/option_type.rs` and `src/sub_enums/*.rs`):
//! - `OptionType`: `is_european`, `is_american`, `is_exotic`,
//!   `is_path_dependent`, `is_multi_asset`.
//! - `BarrierType`: `is_knock_in`, `is_knock_out`, `is_up`, `is_down`.
//! - `AsianAveragingType`: `is_arithmetic`, `is_geometric`.
//! - `RainbowType`: `is_best_of`, `is_worst_of`.
//!
//! `BinaryType` and `LookbackType` expose no classification helpers.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use option_type::prelude::*;
use positive::pos_or_panic;

/// Expected results of the five `OptionType` classification helpers.
struct OptionTypeExpect {
    european: bool,
    american: bool,
    exotic: bool,
    path_dependent: bool,
    multi_asset: bool,
}

/// The full truth table: one entry per `OptionType` variant paired with the
/// expected output of every classification helper. Rebuilt on each call
/// because `OptionType` is not `Copy`.
fn option_type_cases() -> Vec<(OptionType, OptionTypeExpect)> {
    vec![
        (
            OptionType::European,
            OptionTypeExpect {
                european: true,
                american: false,
                exotic: false,
                path_dependent: false,
                multi_asset: false,
            },
        ),
        (
            OptionType::American,
            OptionTypeExpect {
                european: false,
                american: true,
                exotic: false,
                path_dependent: false,
                multi_asset: false,
            },
        ),
        (
            OptionType::Bermuda {
                exercise_dates: vec![pos_or_panic!(30.0), pos_or_panic!(60.0)],
            },
            OptionTypeExpect {
                european: false,
                american: false,
                exotic: true,
                path_dependent: false,
                multi_asset: false,
            },
        ),
        (
            OptionType::Asian {
                averaging_type: AsianAveragingType::Arithmetic,
            },
            OptionTypeExpect {
                european: false,
                american: false,
                exotic: true,
                path_dependent: true,
                multi_asset: false,
            },
        ),
        (
            OptionType::Barrier {
                barrier_type: BarrierType::UpAndIn,
                barrier_level: pos_or_panic!(120.0),
                rebate: None,
            },
            OptionTypeExpect {
                european: false,
                american: false,
                exotic: true,
                path_dependent: true,
                multi_asset: false,
            },
        ),
        (
            OptionType::Binary {
                binary_type: BinaryType::CashOrNothing,
            },
            OptionTypeExpect {
                european: false,
                american: false,
                exotic: true,
                path_dependent: false,
                multi_asset: false,
            },
        ),
        (
            OptionType::Lookback {
                lookback_type: LookbackType::FixedStrike,
            },
            OptionTypeExpect {
                european: false,
                american: false,
                exotic: true,
                path_dependent: true,
                multi_asset: false,
            },
        ),
        (
            OptionType::Compound {
                underlying_option: Box::new(OptionType::European),
            },
            OptionTypeExpect {
                european: false,
                american: false,
                exotic: true,
                path_dependent: false,
                multi_asset: false,
            },
        ),
        (
            OptionType::Chooser {
                choice_date: pos_or_panic!(30.0),
            },
            OptionTypeExpect {
                european: false,
                american: false,
                exotic: true,
                path_dependent: false,
                multi_asset: false,
            },
        ),
        (
            OptionType::Cliquet {
                reset_dates: vec![pos_or_panic!(30.0), pos_or_panic!(60.0)],
            },
            OptionTypeExpect {
                european: false,
                american: false,
                exotic: true,
                path_dependent: true,
                multi_asset: false,
            },
        ),
        (
            OptionType::Rainbow {
                num_assets: 2,
                rainbow_type: RainbowType::BestOf,
            },
            OptionTypeExpect {
                european: false,
                american: false,
                exotic: true,
                path_dependent: false,
                multi_asset: true,
            },
        ),
        (
            OptionType::Spread {
                second_asset: pos_or_panic!(90.0),
            },
            OptionTypeExpect {
                european: false,
                american: false,
                exotic: true,
                path_dependent: false,
                multi_asset: true,
            },
        ),
        (
            OptionType::Quanto {
                exchange_rate: pos_or_panic!(1.5),
            },
            OptionTypeExpect {
                european: false,
                american: false,
                exotic: true,
                path_dependent: false,
                multi_asset: false,
            },
        ),
        (
            OptionType::Exchange {
                second_asset: pos_or_panic!(110.0),
            },
            OptionTypeExpect {
                european: false,
                american: false,
                exotic: true,
                path_dependent: false,
                multi_asset: true,
            },
        ),
        (
            OptionType::Power {
                exponent: pos_or_panic!(2.0),
            },
            OptionTypeExpect {
                european: false,
                american: false,
                exotic: true,
                path_dependent: false,
                multi_asset: false,
            },
        ),
    ]
}

#[test]
fn test_option_type_is_european_matches_truth_table() {
    for (opt, expect) in option_type_cases() {
        assert_eq!(
            opt.is_european(),
            expect.european,
            "is_european mismatch for {opt:?}"
        );
    }
}

#[test]
fn test_option_type_is_american_matches_truth_table() {
    for (opt, expect) in option_type_cases() {
        assert_eq!(
            opt.is_american(),
            expect.american,
            "is_american mismatch for {opt:?}"
        );
    }
}

#[test]
fn test_option_type_is_exotic_matches_truth_table() {
    for (opt, expect) in option_type_cases() {
        assert_eq!(
            opt.is_exotic(),
            expect.exotic,
            "is_exotic mismatch for {opt:?}"
        );
    }
}

#[test]
fn test_option_type_is_path_dependent_matches_truth_table() {
    for (opt, expect) in option_type_cases() {
        assert_eq!(
            opt.is_path_dependent(),
            expect.path_dependent,
            "is_path_dependent mismatch for {opt:?}"
        );
    }
}

#[test]
fn test_option_type_is_multi_asset_matches_truth_table() {
    for (opt, expect) in option_type_cases() {
        assert_eq!(
            opt.is_multi_asset(),
            expect.multi_asset,
            "is_multi_asset mismatch for {opt:?}"
        );
    }
}

// ---------------------------------------------------------------------------
// BarrierType
// ---------------------------------------------------------------------------

#[test]
fn test_barrier_type_classification_matches_truth_table() {
    // (variant, is_knock_in, is_knock_out, is_up, is_down)
    let cases = [
        (BarrierType::UpAndIn, true, false, true, false),
        (BarrierType::UpAndOut, false, true, true, false),
        (BarrierType::DownAndIn, true, false, false, true),
        (BarrierType::DownAndOut, false, true, false, true),
    ];
    for (barrier, knock_in, knock_out, up, down) in cases {
        assert_eq!(
            barrier.is_knock_in(),
            knock_in,
            "is_knock_in for {barrier:?}"
        );
        assert_eq!(
            barrier.is_knock_out(),
            knock_out,
            "is_knock_out for {barrier:?}"
        );
        assert_eq!(barrier.is_up(), up, "is_up for {barrier:?}");
        assert_eq!(barrier.is_down(), down, "is_down for {barrier:?}");
    }
}

// ---------------------------------------------------------------------------
// AsianAveragingType
// ---------------------------------------------------------------------------

#[test]
fn test_asian_averaging_type_classification_matches_truth_table() {
    // (variant, is_arithmetic, is_geometric)
    let cases = [
        (AsianAveragingType::Arithmetic, true, false),
        (AsianAveragingType::Geometric, false, true),
    ];
    for (averaging, arithmetic, geometric) in cases {
        assert_eq!(
            averaging.is_arithmetic(),
            arithmetic,
            "is_arithmetic for {averaging:?}"
        );
        assert_eq!(
            averaging.is_geometric(),
            geometric,
            "is_geometric for {averaging:?}"
        );
    }
}

// ---------------------------------------------------------------------------
// RainbowType
// ---------------------------------------------------------------------------

#[test]
fn test_rainbow_type_classification_matches_truth_table() {
    // (variant, is_best_of, is_worst_of)
    let cases = [
        (RainbowType::BestOf, true, false),
        (RainbowType::WorstOf, false, true),
    ];
    for (rainbow, best_of, worst_of) in cases {
        assert_eq!(rainbow.is_best_of(), best_of, "is_best_of for {rainbow:?}");
        assert_eq!(
            rainbow.is_worst_of(),
            worst_of,
            "is_worst_of for {rainbow:?}"
        );
    }
}
