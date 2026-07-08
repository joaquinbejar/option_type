//! Basic usage of the [`OptionType`] enum: constructing the standard option
//! types (European and American), querying the classification helpers, and
//! printing their `Display` labels.
//!
//! Run with:
//!
//! ```bash
//! cargo run --example basic_usage
//! ```

use option_type::prelude::*;

/// Print an option's `Display` label and every classification flag.
fn describe(option: &OptionType) {
    println!("{option}");
    println!("  is_european:       {}", option.is_european());
    println!("  is_american:       {}", option.is_american());
    println!("  is_exotic:         {}", option.is_exotic());
    println!("  is_path_dependent: {}", option.is_path_dependent());
    println!("  is_multi_asset:    {}", option.is_multi_asset());
    println!();
}

fn main() {
    // Standard vanilla option types.
    let european = OptionType::European;
    let american = OptionType::American;

    // `OptionType::default()` is `European` by explicit design.
    let default_option = OptionType::default();

    println!("== Standard option types ==");
    println!("Display (European): {european}");
    println!("Display (American): {american}");
    println!("Display (default):  {default_option}");
    println!("default == European: {}", default_option == european);
    println!();

    println!("== Classification helpers ==");
    describe(&european);
    describe(&american);
}
