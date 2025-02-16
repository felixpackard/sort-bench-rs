use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};
use rand::{Rng, SeedableRng};

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("algorithms");

    macro_rules! bench {
        ($algorithm:ident, $size:ident) => {
            group.bench_function(BenchmarkId::new(stringify!($algorithm), $size), |b| {
                let rng = rand::rngs::SmallRng::seed_from_u64(0);
                b.iter_batched_ref(
                    || -> Vec<i32> {
                        rng.clone()
                            .sample_iter(rand::distr::StandardUniform)
                            .take($size)
                            .collect()
                    },
                    |v| sorting_algorithms::algorithms::$algorithm::sort(v),
                    BatchSize::SmallInput,
                )
            });
        };
    }

    for i in [10, 10_000, 100_000, 1_000_000] {
        bench!(std, i);
        bench!(quicksort, i);

        if i <= 10_000 {
            bench!(selection_sort, i);
        }
    }

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
