pub fn sort<T: Ord + Copy + std::fmt::Display + std::fmt::Debug>(v: &mut [T]) {
    let n = v.len();
    if n <= 1 {
        return;
    }

    let mut width = 1;
    let mut buffer = vec![v[0]; n];

    while width < n {
        for left in (0..n - 1).step_by(2 * width) {
            let mid = std::cmp::min(left + width, n);
            let right = std::cmp::min(left + 2 * width, n);

            if right - left < 20 {
                crate::utils::insertion_sort(&mut v[left..right]);
                continue;
            }

            let left_slice = &v[left..mid];
            let right_slice = &v[mid..right];

            merge(left_slice, right_slice, &mut buffer[left..right]);

            v[left..right].copy_from_slice(&buffer[left..right]);
        }

        width *= 2;
    }
}

fn merge<T: Ord + Copy + std::fmt::Display>(left_half: &[T], right_half: &[T], v: &mut [T]) {
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

#[cfg(test)]
mod tests {
    #[test]
    fn test_sort() {
        let mut data = crate::utils::rand_data(10_000);
        super::sort(&mut data);
        crate::utils::assert_sorted(&data);
    }
}
