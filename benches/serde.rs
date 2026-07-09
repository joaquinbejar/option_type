//! JSON serialization / deserialization benchmarks.
//!
//! Covers `serde_json::to_string` and `serde_json::from_str` round-trips for a
//! representative [`OptionType`] variant from each family:
//!
//! - Standard: `European`.
//! - Path-dependent: `Asian`, `Barrier`, `Cliquet`.
//! - Multi-asset: `Rainbow`, `Spread`.
//! - Structural: `Bermuda`, `Binary`.
//! - Modified payoff: `Power`.
//!
//! The `Compound` variant is intentionally omitted: it is `#[serde(skip)]` and
//! therefore not serializable. `from_str` inputs are serialized once up front so
//! the benchmark measures only deserialization. Inputs and results are
//! `black_box`ed.

// `criterion_group!` expands to an undocumented `pub fn`; the crate's
// `missing_docs = "warn"` lint would otherwise flag it under `-D warnings`.
#![allow(missing_docs)]
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};
use option_type::{AsianAveragingType, BarrierType, BinaryType, OptionType, RainbowType};

/// Builds the representative variant set covering every serializable family.
fn variants() -> Vec<(&'static str, OptionType)> {
    vec![
        ("european", OptionType::European),
        (
            "asian",
            OptionType::Asian {
                averaging_type: AsianAveragingType::Arithmetic,
            },
        ),
        (
            "barrier",
            OptionType::Barrier {
                barrier_type: BarrierType::UpAndIn,
                barrier_level: 120.0,
                rebate: Some(5.0),
            },
        ),
        (
            "cliquet",
            OptionType::Cliquet {
                reset_dates: vec![30.0, 60.0, 90.0, 120.0],
            },
        ),
        (
            "rainbow",
            OptionType::Rainbow {
                num_assets: 3,
                rainbow_type: RainbowType::WorstOf,
            },
        ),
        ("spread", OptionType::Spread { second_asset: 90.0 }),
        (
            "bermuda",
            OptionType::Bermuda {
                exercise_dates: vec![30.0, 60.0, 90.0, 120.0],
            },
        ),
        (
            "binary",
            OptionType::Binary {
                binary_type: BinaryType::CashOrNothing,
            },
        ),
        ("power", OptionType::Power { exponent: 2.0 }),
    ]
}

fn bench_to_string(c: &mut Criterion) {
    let variants = variants();
    let mut g = c.benchmark_group("serde/to_string");
    for (name, variant) in &variants {
        g.bench_function(*name, |b| {
            b.iter(|| black_box(serde_json::to_string(black_box(variant))))
        });
    }
    g.finish();
}

fn bench_from_str(c: &mut Criterion) {
    let encoded: Vec<(&'static str, String)> = variants()
        .into_iter()
        .map(|(name, variant)| {
            (
                name,
                serde_json::to_string(&variant).expect("bench variant must serialize"),
            )
        })
        .collect();

    let mut g = c.benchmark_group("serde/from_str");
    for (name, json) in &encoded {
        g.bench_function(*name, |b| {
            b.iter(|| black_box(serde_json::from_str::<OptionType>(black_box(json))))
        });
    }
    g.finish();
}

criterion_group!(benches, bench_to_string, bench_from_str);
criterion_main!(benches);
