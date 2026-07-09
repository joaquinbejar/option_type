//! JSON serialization and deserialization round-trips for a selection of
//! [`OptionType`] variants using `serde_json`, printing the produced JSON and
//! confirming that the value survives the round-trip unchanged.
//!
//! Run with:
//!
//! ```bash
//! cargo run --example serde_roundtrip
//! ```

// Examples may use `.unwrap()` for brevity; the crate `[lints]` deny it for
// production code, so opt out explicitly here.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use option_type::prelude::*;
use positive::pos_or_panic;

/// Serialize `option` to JSON, deserialize it back, and print both the JSON
/// and whether the round-trip preserved the value.
fn roundtrip(label: &str, option: &OptionType) {
    let json = serde_json::to_string(option).unwrap();
    let parsed: OptionType = serde_json::from_str(&json).unwrap();
    println!("{label}");
    println!("  json:  {json}");
    println!("  back:  {parsed}");
    println!("  equal: {}", *option == parsed);
    println!();
}

fn main() {
    println!("== serde JSON round-trips ==\n");

    roundtrip("European", &OptionType::European);
    roundtrip("American", &OptionType::American);
    roundtrip(
        "Asian (Geometric)",
        &OptionType::Asian {
            averaging_type: AsianAveragingType::Geometric,
        },
    );
    roundtrip(
        "Barrier (DownAndOut)",
        &OptionType::Barrier {
            barrier_type: BarrierType::DownAndOut,
            barrier_level: pos_or_panic!(90.0),
            rebate: None,
        },
    );
    roundtrip(
        "Binary (CashOrNothing)",
        &OptionType::Binary {
            binary_type: BinaryType::CashOrNothing,
        },
    );
    roundtrip(
        "Lookback (FixedStrike)",
        &OptionType::Lookback {
            lookback_type: LookbackType::FixedStrike,
        },
    );
    roundtrip(
        "Rainbow (WorstOf)",
        &OptionType::Rainbow {
            num_assets: 2,
            rainbow_type: RainbowType::WorstOf,
        },
    );
    roundtrip(
        "Bermuda",
        &OptionType::Bermuda {
            exercise_dates: vec![
                pos_or_panic!(30.0),
                pos_or_panic!(60.0),
                pos_or_panic!(90.0),
            ],
        },
    );
    roundtrip(
        "Power",
        &OptionType::Power {
            exponent: pos_or_panic!(2.0),
        },
    );
}
