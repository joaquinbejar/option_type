//! Building an [`OptionBasicType`] — the lightweight, lifetime-parameterized
//! reference struct — from the core financial primitive types
//! (`financial_types::{OptionStyle, Side}`, `positive::Positive`,
//! `expiration_date::ExpirationDate`) and printing its borrowed fields.
//!
//! Run with:
//!
//! ```bash
//! cargo run --example option_basic_type
//! ```

// Examples may use `.unwrap()` for brevity; the crate `[lints]` deny it for
// production code, so opt out explicitly here.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use expiration_date::ExpirationDate;
use financial_types::{OptionStyle, Side};
use option_type::prelude::*;
use positive::Positive;

fn main() {
    // `OptionBasicType` borrows all of its fields, so the owned values must
    // outlive it — bind them to locals first, then take references.
    let option_style = OptionStyle::Call;
    let side = Side::Long;
    let strike_price = Positive::new(100.0).unwrap();
    let expiration_date = ExpirationDate::Days(Positive::new(30.0).unwrap());

    let basic = OptionBasicType {
        option_style: &option_style,
        side: &side,
        strike_price: &strike_price,
        expiration_date: &expiration_date,
    };

    println!("== OptionBasicType (borrowed fields) ==");
    println!("  option_style:    {}", basic.option_style);
    println!("  side:            {}", basic.side);
    println!("  strike_price:    {}", basic.strike_price);
    println!("  expiration_date: {}", basic.expiration_date);
    println!();

    // `OptionBasicType` is `Copy` (all fields are shared references), so it can
    // be copied freely and cheaply.
    let copied = basic;
    println!("Copied struct equals original: {}", copied == basic);
    println!("Debug: {basic:?}");
}
