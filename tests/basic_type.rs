//! Integration tests: [`OptionBasicType`] construction and its
//! `Copy`/`Clone`/`Eq`/`Hash`/`Debug` semantics, exercised as a downstream
//! user would (through the public surface and its regular dependencies).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use expiration_date::ExpirationDate;
use financial_types::{OptionStyle, Side};
use option_type::prelude::*;
use positive::Positive;
use std::collections::HashSet;

#[test]
fn test_option_basic_type_construction_exposes_fields() {
    let style = OptionStyle::Call;
    let side = Side::Long;
    let strike = Positive::new(100.0).expect("100.0 is positive");
    let expiration = ExpirationDate::Days(Positive::new(30.0).expect("30.0 is positive"));

    let basic = OptionBasicType {
        option_style: &style,
        side: &side,
        strike_price: &strike,
        expiration_date: &expiration,
    };

    assert_eq!(*basic.option_style, OptionStyle::Call);
    assert_eq!(*basic.side, Side::Long);
    assert_eq!(*basic.strike_price, strike);
    assert_eq!(*basic.expiration_date, expiration);
}

#[test]
#[allow(clippy::clone_on_copy)] // deliberately exercising the `Clone` impl on a `Copy` type
fn test_option_basic_type_copy_and_clone_preserve_equality() {
    let style = OptionStyle::Put;
    let side = Side::Short;
    let strike = Positive::new(150.0).expect("150.0 is positive");
    let expiration = ExpirationDate::Days(Positive::new(45.0).expect("45.0 is positive"));

    let basic = OptionBasicType {
        option_style: &style,
        side: &side,
        strike_price: &strike,
        expiration_date: &expiration,
    };

    // `OptionBasicType` is `Copy`: the original stays usable after the copy.
    let copied = basic;
    assert_eq!(basic, copied);

    // `Clone` yields an equal value as well.
    let cloned = basic.clone();
    assert_eq!(basic, cloned);
}

#[test]
fn test_option_basic_type_equality_distinguishes_fields() {
    let call = OptionStyle::Call;
    let put = OptionStyle::Put;
    let side = Side::Long;
    let strike = Positive::new(100.0).expect("100.0 is positive");
    let expiration = ExpirationDate::Days(Positive::new(30.0).expect("30.0 is positive"));

    let a = OptionBasicType {
        option_style: &call,
        side: &side,
        strike_price: &strike,
        expiration_date: &expiration,
    };
    let same = OptionBasicType {
        option_style: &call,
        side: &side,
        strike_price: &strike,
        expiration_date: &expiration,
    };
    let different = OptionBasicType {
        option_style: &put,
        side: &side,
        strike_price: &strike,
        expiration_date: &expiration,
    };

    assert_eq!(a, same);
    assert_ne!(a, different);
}

#[test]
fn test_option_basic_type_hash_dedups_in_hash_set() {
    let style = OptionStyle::Call;
    let side = Side::Long;
    let strike = Positive::new(100.0).expect("100.0 is positive");
    let expiration = ExpirationDate::Days(Positive::new(30.0).expect("30.0 is positive"));

    let put = OptionStyle::Put;
    let other_strike = Positive::new(200.0).expect("200.0 is positive");

    let equal_a = OptionBasicType {
        option_style: &style,
        side: &side,
        strike_price: &strike,
        expiration_date: &expiration,
    };
    let equal_b = OptionBasicType {
        option_style: &style,
        side: &side,
        strike_price: &strike,
        expiration_date: &expiration,
    };
    let distinct = OptionBasicType {
        option_style: &put,
        side: &side,
        strike_price: &other_strike,
        expiration_date: &expiration,
    };

    let mut set = HashSet::new();
    set.insert(equal_a);
    set.insert(equal_b); // equal to `equal_a`, must not add a new entry
    assert_eq!(set.len(), 1);

    set.insert(distinct);
    assert_eq!(set.len(), 2);
    assert!(set.contains(&equal_a));
    assert!(set.contains(&distinct));
}

#[test]
fn test_option_basic_type_debug_is_non_empty() {
    let style = OptionStyle::Call;
    let side = Side::Long;
    let strike = Positive::new(100.0).expect("100.0 is positive");
    let expiration = ExpirationDate::Days(Positive::new(30.0).expect("30.0 is positive"));

    let basic = OptionBasicType {
        option_style: &style,
        side: &side,
        strike_price: &strike,
        expiration_date: &expiration,
    };

    let debug = format!("{basic:?}");
    assert!(!debug.is_empty(), "Debug output must not be empty");
    assert!(
        debug.contains("OptionBasicType"),
        "Debug output should name the struct, got: {debug}"
    );
}
