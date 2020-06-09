use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use benches::*;

pub fn rng_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Rng");
    group.sample_size(20);
    group.bench_function(BenchmarkId::new("Small", "u"), |b| {
        b.iter(|| small_rng())
    });
    group.bench_function(BenchmarkId::new("Thread", "s"), |b| {
        b.iter(|| thread_rng_())
    });
}

criterion_group!(bench_grp, rng_benchmark);
criterion_main!(bench_grp);