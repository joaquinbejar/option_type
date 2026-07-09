//! `Display` rendering benchmarks.
//!
//! Measures `to_string()` for the [`OptionType`] `Display` impl across
//! representative variants, including the payload-carrying ones whose labels
//! interpolate their fields — `Barrier` (enum + level + optional rebate),
//! `Bermuda` (a `Vec<f64>` of exercise dates), and `Rainbow` (asset count +
//! type). A second group covers the `#[inline]` `Display` impls on the 1-byte
//! leaf sub-enums. Inputs and rendered strings are `black_box`ed.

// `criterion_group!` expands to an undocumented `pub fn`; the crate's
// `missing_docs = "warn"` lint would otherwise flag it under `-D warnings`.
#![allow(missing_docs)]

use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};
use option_type::{
    AsianAveragingType, BarrierType, BinaryType, LookbackType, OptionType, RainbowType,
};

fn bench_option_type_display(c: &mut Criterion) {
    let european = OptionType::European;
    let american = OptionType::American;
    let asian = OptionType::Asian {
        averaging_type: AsianAveragingType::Geometric,
    };
    let barrier = OptionType::Barrier {
        barrier_type: BarrierType::UpAndIn,
        barrier_level: 120.0,
        rebate: Some(5.0),
    };
    let bermuda = OptionType::Bermuda {
        exercise_dates: vec![30.0, 60.0, 90.0, 120.0],
    };
    let rainbow = OptionType::Rainbow {
        num_assets: 3,
        rainbow_type: RainbowType::WorstOf,
    };
    let power = OptionType::Power { exponent: 2.0 };

    let mut g = c.benchmark_group("display/option_type");
    g.bench_function("european", |b| {
        b.iter(|| black_box(black_box(&european).to_string()))
    });
    g.bench_function("american", |b| {
        b.iter(|| black_box(black_box(&american).to_string()))
    });
    g.bench_function("asian", |b| {
        b.iter(|| black_box(black_box(&asian).to_string()))
    });
    g.bench_function("barrier", |b| {
        b.iter(|| black_box(black_box(&barrier).to_string()))
    });
    g.bench_function("bermuda", |b| {
        b.iter(|| black_box(black_box(&bermuda).to_string()))
    });
    g.bench_function("rainbow", |b| {
        b.iter(|| black_box(black_box(&rainbow).to_string()))
    });
    g.bench_function("power", |b| {
        b.iter(|| black_box(black_box(&power).to_string()))
    });
    g.finish();
}

fn bench_leaf_enum_display(c: &mut Criterion) {
    let averaging = AsianAveragingType::Arithmetic;
    let barrier = BarrierType::DownAndOut;
    let binary = BinaryType::CashOrNothing;
    let lookback = LookbackType::FloatingStrike;

    let mut g = c.benchmark_group("display/leaf_enum");
    g.bench_function("asian_averaging_type", |b| {
        b.iter(|| black_box(black_box(averaging).to_string()))
    });
    g.bench_function("barrier_type", |b| {
        b.iter(|| black_box(black_box(barrier).to_string()))
    });
    g.bench_function("binary_type", |b| {
        b.iter(|| black_box(black_box(binary).to_string()))
    });
    g.bench_function("lookback_type", |b| {
        b.iter(|| black_box(black_box(lookback).to_string()))
    });
    g.finish();
}

criterion_group!(benches, bench_option_type_display, bench_leaf_enum_display);
criterion_main!(benches);
