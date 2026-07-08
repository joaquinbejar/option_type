//! The lightweight borrowing [`OptionBasicType`] struct.

use expiration_date::ExpirationDate;
use financial_types::{OptionStyle, Side};
use positive::Positive;
use serde::Serialize;

/// A structure representing the basic properties of an option in financial terms.
///
/// This structure is designed to be lightweight and provides essential details
/// about an options contract through borrowed references.
///
/// # Examples
///
/// ```rust
/// use option_type::OptionBasicType;
/// use financial_types::{OptionStyle, Side};
/// use positive::Positive;
/// use expiration_date::ExpirationDate;
/// use positive::pos_or_panic;
///
/// let european_call_option = OptionBasicType {
///     option_style: &OptionStyle::Call,
///     side: &Side::Long,
///     strike_price: &Positive::new(100.0).unwrap(),
///     expiration_date: &ExpirationDate::Days(pos_or_panic!(30.0)),
/// };
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct OptionBasicType<'a> {
    /// The style of the option (Call or Put).
    pub option_style: &'a OptionStyle,
    /// The side of the position (Long or Short).
    pub side: &'a Side,
    /// The strike price of the option, guaranteed to be positive.
    pub strike_price: &'a Positive,
    /// The expiration date of the option.
    pub expiration_date: &'a ExpirationDate,
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic, clippy::expect_used)]
mod tests {
    use super::*;
    use positive::pos_or_panic;

    #[test]
    fn test_creation() {
        let style = OptionStyle::Call;
        let side = Side::Long;
        let strike = pos_or_panic!(100.0);
        let expiration = ExpirationDate::Days(pos_or_panic!(30.0));

        let basic = OptionBasicType {
            option_style: &style,
            side: &side,
            strike_price: &strike,
            expiration_date: &expiration,
        };

        assert_eq!(*basic.option_style, OptionStyle::Call);
        assert_eq!(*basic.side, Side::Long);
        assert_eq!(*basic.strike_price, strike);
    }

    #[test]
    fn test_equality() {
        let style = OptionStyle::Put;
        let side = Side::Short;
        let strike = pos_or_panic!(200.0);
        let expiration = ExpirationDate::Days(pos_or_panic!(60.0));

        let basic1 = OptionBasicType {
            option_style: &style,
            side: &side,
            strike_price: &strike,
            expiration_date: &expiration,
        };

        let basic2 = OptionBasicType {
            option_style: &style,
            side: &side,
            strike_price: &strike,
            expiration_date: &expiration,
        };

        assert_eq!(basic1, basic2);
    }

    #[test]
    fn test_copy() {
        let style = OptionStyle::Call;
        let side = Side::Long;
        let strike = pos_or_panic!(150.0);
        let expiration = ExpirationDate::Days(pos_or_panic!(45.0));

        let basic = OptionBasicType {
            option_style: &style,
            side: &side,
            strike_price: &strike,
            expiration_date: &expiration,
        };

        let copied = basic;
        assert_eq!(basic, copied);
    }

    #[test]
    fn test_hash() {
        use std::collections::HashSet;

        let style = OptionStyle::Call;
        let side = Side::Long;
        let strike = pos_or_panic!(100.0);
        let expiration = ExpirationDate::Days(pos_or_panic!(30.0));

        let basic = OptionBasicType {
            option_style: &style,
            side: &side,
            strike_price: &strike,
            expiration_date: &expiration,
        };

        let mut set = HashSet::new();
        set.insert(basic);
        set.insert(basic); // duplicate
        assert_eq!(set.len(), 1);
    }
}
