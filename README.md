[![Dual License](https://img.shields.io/badge/license-MIT-blue)](./LICENSE)
[![Crates.io](https://img.shields.io/crates/v/option_type.svg)](https://crates.io/crates/option_type)
[![Downloads](https://img.shields.io/crates/d/option_type.svg)](https://crates.io/crates/option_type)
[![Stars](https://img.shields.io/github/stars/joaquinbejar/option_type.svg)](https://github.com/joaquinbejar/option_type/stargazers)
[![Issues](https://img.shields.io/github/issues/joaquinbejar/option_type.svg)](https://github.com/joaquinbejar/option_type/issues)
[![PRs](https://img.shields.io/github/issues-pr/joaquinbejar/option_type.svg)](https://github.com/joaquinbejar/option_type/pulls)
[![Build Status](https://img.shields.io/github/workflow/status/joaquinbejar/option_type/CI)](https://github.com/joaquinbejar/option_type/actions)
[![Coverage](https://img.shields.io/codecov/c/github/joaquinbejar/option_type)](https://codecov.io/gh/joaquinbejar/option_type)
[![Dependencies](https://img.shields.io/librariesio/github/joaquinbejar/option_type)](https://libraries.io/github/joaquinbejar/option_type)
[![Documentation](https://img.shields.io/badge/docs-latest-blue.svg)](https://docs.rs/option_type)
[![Wiki](https://img.shields.io/badge/wiki-latest-blue.svg)](https://deepwiki.com/joaquinbejar/option_type)


## Option Type

Option contract type definitions including standard and exotic options for Rust.

### Overview

`option_type` is a Rust crate providing a comprehensive enum-based classification
of financial option contracts. It covers both standard vanilla options (European,
American) and a wide range of exotic option types.

All leaf sub-enums use `#[repr(u8)]` for compact memory layout (1 byte each).
Pure helper methods are annotated with `#[must_use]` and `#[inline]`.

### Supported Option Types

| Category | Types |
|---|---|
| **Standard** | European, American |
| **Path-dependent** | Asian, Barrier, Lookback, Cliquet |
| **Multi-asset** | Rainbow, Spread, Exchange |
| **Structural** | Compound, Chooser, Binary, Bermuda |
| **Modified payoff** | Power, Quanto |

### Sub-type Enums

| Enum | Variants | Description |
|---|---|---|
| `AsianAveragingType` | Arithmetic, Geometric | Averaging method for Asian options |
| `BarrierType` | UpAndIn, UpAndOut, DownAndIn, DownAndOut | Barrier trigger conditions |
| `BinaryType` | CashOrNothing, AssetOrNothing, Gap | Binary option payout types |
| `LookbackType` | FixedStrike, FloatingStrike | Lookback strike determination |
| `RainbowType` | BestOf, WorstOf | Multi-asset selection method |

### Features

- **Comprehensive**: 15 option type variants covering vanilla and exotic options
- **Compact sub-enums**: All leaf enums are `#[repr(u8)]` — 1 byte each
- **Safe**: `#[must_use]` on all pure helper methods
- **Serde**: Full serialization/deserialization support
- **OpenAPI**: Optional `utoipa` support via feature flag
- **Helpers**: `is_european()`, `is_exotic()`, `is_path_dependent()`, `is_multi_asset()`, and more

### Dependencies

- [`financial_types`](https://crates.io/crates/financial_types) — `OptionStyle`, `Side`
- [`positive`](https://crates.io/crates/positive) — Type-safe positive decimal values
- [`expiration_date`](https://crates.io/crates/expiration_date) — Expiration date handling

### Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
option_type = "0.1"
```

To enable OpenAPI schema support:

```toml
[dependencies]
option_type = { version = "0.1", features = ["utoipa"] }
```

### Quick Start

```rust
use option_type::{OptionType, AsianAveragingType, BarrierType};

// Standard options
let european = OptionType::European;
assert!(european.is_european());
assert!(!european.is_exotic());

// Exotic options
let asian = OptionType::Asian {
    averaging_type: AsianAveragingType::Arithmetic,
};
assert!(asian.is_exotic());
assert!(asian.is_path_dependent());

// Barrier options
let barrier = OptionType::Barrier {
    barrier_type: BarrierType::UpAndIn,
    barrier_level: 120.0,
    rebate: None,
};
assert!(barrier.is_path_dependent());

// Display
assert_eq!(format!("{european}"), "European Option");
```

### API

#### `OptionType`

The main enum classifying option contracts:

```rust
use option_type::OptionType;

let opt = OptionType::default(); // European
assert!(opt.is_european());
assert!(!opt.is_exotic());
assert!(!opt.is_path_dependent());
assert!(!opt.is_multi_asset());
```

Helpers: `is_european()`, `is_american()`, `is_exotic()`, `is_path_dependent()`, `is_multi_asset()`

#### `BarrierType`

```rust
use option_type::BarrierType;

let barrier = BarrierType::UpAndIn;
assert!(barrier.is_knock_in());
assert!(barrier.is_up());
assert!(!barrier.is_knock_out());
assert!(!barrier.is_down());
```

Helpers: `is_knock_in()`, `is_knock_out()`, `is_up()`, `is_down()`

#### `AsianAveragingType`

```rust
use option_type::AsianAveragingType;

let avg = AsianAveragingType::Arithmetic;
assert!(avg.is_arithmetic());
assert!(!avg.is_geometric());
```

Helpers: `is_arithmetic()`, `is_geometric()`

#### `RainbowType`

```rust
use option_type::RainbowType;

let rainbow = RainbowType::BestOf;
assert!(rainbow.is_best_of());
assert!(!rainbow.is_worst_of());
```

Helpers: `is_best_of()`, `is_worst_of()`

#### `OptionBasicType`

A lightweight struct referencing core option properties:

```rust
use option_type::OptionBasicType;
use financial_types::{OptionStyle, Side};
use positive::Positive;
use expiration_date::ExpirationDate;
use positive::pos_or_panic;

let style = OptionStyle::Call;
let side = Side::Long;
let strike = Positive::new(100.0).unwrap();
let expiry = ExpirationDate::Days(pos_or_panic!(30.0));

let basic = OptionBasicType {
    option_style: &style,
    side: &side,
    strike_price: &strike,
    expiration_date: &expiry,
};
```

#### Serialization

```rust
use option_type::{OptionType, AsianAveragingType};

let opt = OptionType::Asian {
    averaging_type: AsianAveragingType::Geometric,
};
let json = serde_json::to_string(&opt).unwrap();
let parsed: OptionType = serde_json::from_str(&json).unwrap();
assert_eq!(opt, parsed);
```

### License

This project is licensed under the MIT License.



## Contribution and Contact

We welcome contributions to this project! If you would like to contribute, please follow these steps:

1. Fork the repository.
2. Create a new branch for your feature or bug fix.
3. Make your changes and ensure that the project still builds and all tests pass.
4. Commit your changes and push your branch to your forked repository.
5. Submit a pull request to the main repository.

If you have any questions, issues, or would like to provide feedback, please feel free to contact the project maintainer:


### **Contact Information**

- **Author**: Joaquín Béjar García
- **Email**: jb@taunais.com
- **Telegram**: [@joaquin_bejar](https://t.me/joaquin_bejar)
- **Repository**: <https://github.com/joaquinbejar/option_type>
- **Documentation**: <https://docs.rs/option_type>

We appreciate your interest and look forward to your contributions!

## ✍️ License

Licensed under **MIT** license
