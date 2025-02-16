use rand::{Rng, SeedableRng};

pub fn rand_data(size: usize) -> Vec<i32> {
    let rng = rand::rngs::SmallRng::seed_from_u64(0);
    rng.sample_iter(rand::distr::StandardUniform)
        .take(size)
        .collect()
}

pub fn assert_sorted<T: PartialOrd + Copy + std::fmt::Display>(v: &[T]) {
    let mut prev = None;
    for i in v {
        if let Some(prev) = prev {
            assert!(
                i >= prev,
                "elements should be sorted in ascending order ({i} is less than {prev})"
            );
        }
        prev = Some(i);
    }
}
