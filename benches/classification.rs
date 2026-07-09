//! Classification-helper benchmarks: the branch-free `is_*` predicates.
//!
//! Covers every `is_*` helper in the crate — 13 total:
//!
//! - [`OptionType`] (5): `is_european`, `is_american`, `is_exotic`,
//!   `is_path_dependent`, `is_multi_asset`.
//! - [`BarrierType`] (4): `is_knock_in`, `is_knock_out`, `is_up`, `is_down`.
//! - [`AsianAveragingType`] (2): `is_arithmetic`, `is_geometric`.
//! - [`RainbowType`] (2): `is_best_of`, `is_worst_of`.
//!
//! Each helper is measured against a representative variant so the branch-free
//! `matches!` lowering is exercised on the hot path. Inputs and results are
//! `black_box`ed to keep the optimizer from folding the predicate to a constant.

// `criterion_group!` expands to an undocumented `pub fn`; the crate's
// `missing_docs = "warn"` lint would otherwise flag it under `-D warnings`.
#![allow(missing_docs)]

use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};
use option_type::{AsianAveragingType, BarrierType, OptionType, RainbowType};

fn bench_option_type(c: &mut Criterion) {
    let european = OptionType::European;
    let american = OptionType::American;
    let asian = OptionType::Asian {
        averaging_type: AsianAveragingType::Arithmetic,
    };
    let barrier = OptionType::Barrier {
        barrier_type: BarrierType::UpAndIn,
        barrier_level: 120.0,
        rebate: Some(5.0),
    };
    let rainbow = OptionType::Rainbow {
        num_assets: 3,
        rainbow_type: RainbowType::BestOf,
    };

    let mut g = c.benchmark_group("classification/option_type");
    g.bench_function("is_european", |b| {
        b.iter(|| black_box(black_box(&european).is_european()))
    });
    g.bench_function("is_american", |b| {
        b.iter(|| black_box(black_box(&american).is_american()))
    });
    g.bench_function("is_exotic", |b| {
        b.iter(|| black_box(black_box(&asian).is_exotic()))
    });
    g.bench_function("is_path_dependent", |b| {
        b.iter(|| black_box(black_box(&barrier).is_path_dependent()))
    });
    g.bench_function("is_multi_asset", |b| {
        b.iter(|| black_box(black_box(&rainbow).is_multi_asset()))
    });
    g.finish();
}

fn bench_barrier_type(c: &mut Criterion) {
    let knock_in = BarrierType::DownAndIn;
    let knock_out = BarrierType::UpAndOut;
    let up = BarrierType::UpAndIn;
    let down = BarrierType::DownAndOut;

    let mut g = c.benchmark_group("classification/barrier_type");
    g.bench_function("is_knock_in", |b| {
        b.iter(|| black_box(black_box(knock_in).is_knock_in()))
    });
    g.bench_function("is_knock_out", |b| {
        b.iter(|| black_box(black_box(knock_out).is_knock_out()))
    });
    g.bench_function("is_up", |b| b.iter(|| black_box(black_box(up).is_up())));
    g.bench_function("is_down", |b| {
        b.iter(|| black_box(black_box(down).is_down()))
    });
    g.finish();
}

fn bench_asian_averaging_type(c: &mut Criterion) {
    let arithmetic = AsianAveragingType::Arithmetic;
    let geometric = AsianAveragingType::Geometric;

    let mut g = c.benchmark_group("classification/asian_averaging_type");
    g.bench_function("is_arithmetic", |b| {
        b.iter(|| black_box(black_box(arithmetic).is_arithmetic()))
    });
    g.bench_function("is_geometric", |b| {
        b.iter(|| black_box(black_box(geometric).is_geometric()))
    });
    g.finish();
}

fn bench_rainbow_type(c: &mut Criterion) {
    let best_of = RainbowType::BestOf;
    let worst_of = RainbowType::WorstOf;

    let mut g = c.benchmark_group("classification/rainbow_type");
    g.bench_function("is_best_of", |b| {
        b.iter(|| black_box(black_box(best_of).is_best_of()))
    });
    g.bench_function("is_worst_of", |b| {
        b.iter(|| black_box(black_box(worst_of).is_worst_of()))
    });
    g.finish();
}

criterion_group!(
    benches,
    bench_option_type,
    bench_barrier_type,
    bench_asian_averaging_type,
    bench_rainbow_type,
);
criterion_main!(benches);
