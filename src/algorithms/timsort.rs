const MIN_MERGE: usize = 32;

pub fn sort<T: Ord + Copy>(v: &mut [T]) {
    let n = v.len();
    if n <= 1 {
        return;
    }

    let mut buffer = vec![v[0]; n];

    let min_run = calc_min_run(n);

    for start in (0..n).step_by(min_run) {
        let end = std::cmp::min(start + min_run, n);
        crate::utils::insertion_sort(&mut v[start..end]);
    }

    let mut size = min_run;
    while size < n {
        for left in (0..n).step_by(size * 2) {
            let mid = std::cmp::min(left + size, n);
            let right = std::cmp::min(left + size * 2, n);

            let left_slice = &v[left..mid];
            let right_slice = &v[mid..right];

            merge(left_slice, right_slice, &mut buffer[left..right]);

            v[left..right].copy_from_slice(&buffer[left..right]);
        }

        size *= 2;
    }
}

fn merge<T: Ord + Copy>(left_half: &[T], right_half: &[T], v: &mut [T]) {
    let mut left_iter = left_half.iter().peekable();
    let mut right_iter = right_half.iter().peekable();
    let mut output = v.iter_mut();

    while let (Some(&left), Some(&right)) = (left_iter.peek(), right_iter.peek()) {
        let next = if left <= right {
            *left_iter.next().unwrap()
        } else {
            *right_iter.next().unwrap()
        };
        *output.next().unwrap() = next;
    }

    let left_remaining = &left_half[left_half.len() - left_iter.count()..];
    let right_remaining = &right_half[right_half.len() - right_iter.count()..];

    if !left_remaining.is_empty() {
        let pos = v.len() - left_remaining.len();
        v[pos..].copy_from_slice(left_remaining);
    } else if !right_remaining.is_empty() {
        let pos = v.len() - right_remaining.len();
        v[pos..].copy_from_slice(right_remaining);
    }
}

fn calc_min_run(mut n: usize) -> usize {
    let mut r = 0;
    while n >= MIN_MERGE {
        r |= n & 1;
        n >>= 1;
    }
    n + r
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_sort() {
        let mut data = crate::utils::rand_data(10_000);
        super::sort(&mut data);
        crate::utils::assert_sorted(&data);
    }
}
