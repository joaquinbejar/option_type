//! Constructing exotic [`OptionType`] variants (Asian, Barrier, Lookback,
//! Rainbow, Bermuda) with realistic payloads, then printing their
//! classification flags and `Display` labels.
//!
//! Run with:
//!
//! ```bash
//! cargo run --example exotic_options
//! ```

use option_type::prelude::*;
use positive::pos_or_panic;

/// Print an option's `Display` label and its exotic-classification flags.
fn describe(option: &OptionType) {
    println!("{option}");
    println!("  is_exotic:         {}", option.is_exotic());
    println!("  is_path_dependent: {}", option.is_path_dependent());
    println!("  is_multi_asset:    {}", option.is_multi_asset());
    println!();
}

fn main() {
    // Asian option averaging the underlying price arithmetically.
    let asian = OptionType::Asian {
        averaging_type: AsianAveragingType::Arithmetic,
    };

    // Up-and-in barrier triggered at a level of 120.0 with a 5.0 rebate.
    let barrier = OptionType::Barrier {
        barrier_type: BarrierType::UpAndIn,
        barrier_level: pos_or_panic!(120.0),
        rebate: Some(pos_or_panic!(5.0)),
    };

    // Floating-strike lookback option.
    let lookback = OptionType::Lookback {
        lookback_type: LookbackType::FloatingStrike,
    };

    // Best-of rainbow option over three underlying assets.
    let rainbow = OptionType::Rainbow {
        num_assets: 3,
        rainbow_type: RainbowType::BestOf,
    };

    // Bermuda option exercisable at 30, 60, and 90 days to expiry.
    let bermuda = OptionType::Bermuda {
        exercise_dates: vec![
            pos_or_panic!(30.0),
            pos_or_panic!(60.0),
            pos_or_panic!(90.0),
        ],
    };

    println!("== Exotic option types ==\n");
    describe(&asian);
    describe(&barrier);
    describe(&lookback);
    describe(&rainbow);
    describe(&bermuda);

    // Leaf sub-enum helpers, illustrated on the barrier type.
    if let OptionType::Barrier { barrier_type, .. } = &barrier {
        println!("== Barrier sub-type helpers ==");
        println!("  barrier:      {barrier_type}");
        println!("  is_knock_in:  {}", barrier_type.is_knock_in());
        println!("  is_knock_out: {}", barrier_type.is_knock_out());
        println!("  is_up:        {}", barrier_type.is_up());
        println!("  is_down:      {}", barrier_type.is_down());
        println!();
    }

    // Leaf sub-enum helpers, illustrated on the rainbow type.
    if let OptionType::Rainbow { rainbow_type, .. } = &rainbow {
        println!("== Rainbow sub-type helpers ==");
        println!("  rainbow:      {rainbow_type:?}");
        println!("  is_best_of:   {}", rainbow_type.is_best_of());
        println!("  is_worst_of:  {}", rainbow_type.is_worst_of());
    }
}
