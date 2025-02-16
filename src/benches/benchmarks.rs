use sorting_algorithms::algorithms;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    macro_rules! bench {
        ($algorithm:ident, $size:literal) => {
            c.bench_function(
                format!("{} {}", stringify!($algorithm), $size).as_str(),
                |b| {
                    b.iter(|| {
                        algorithms::$algorithm::sort(black_box(
                            &mut sorting_algorithms::utils::rand_data($size),
                        ))
                    })
                },
            );
        };
    }

    macro_rules! bench_all {
        ($algorithm:ident) => {
            bench!($algorithm, 10);
            bench!($algorithm, 10_000);
            bench!($algorithm, 100_000);
            bench!($algorithm, 1_000_000);
        };
    }

    bench_all!(quicksort);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
