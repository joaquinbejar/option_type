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


{{readme}}


## Examples

Runnable examples live under [`examples/`](./examples). Each prints a short,
self-explanatory walkthrough of one part of the API:

| Example | Description | Command |
|---|---|---|
| `basic_usage` | Construct European/American options, query the `is_*` helpers, print `Display` labels | `cargo run --example basic_usage` |
| `exotic_options` | Build Asian, Barrier, Lookback, Rainbow, and Bermuda options with realistic payloads and print their classification | `cargo run --example exotic_options` |
| `serde_roundtrip` | Serialize a variety of variants to JSON and deserialize them back, printing the JSON | `cargo run --example serde_roundtrip` |
| `option_basic_type` | Assemble an `OptionBasicType` from `OptionStyle`, `Side`, `Positive`, and `ExpirationDate`, printing its borrowed fields | `cargo run --example option_basic_type` |


## MSRV

The Minimum Supported Rust Version (MSRV) is **1.85** (required by `edition = "2024"`
and by this crate's direct dependencies). CI enforces the MSRV via a dedicated
`msrv.yml` workflow that builds and tests the crate on the pinned toolchain.

MSRV bumps are considered breaking changes and require a major (or pre-1.0
minor) version bump.


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
