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

            let left_slice = &v[left..mid];
            let right_slice = &v[mid..right];

            merge(left_slice, right_slice, &mut buffer[left..right]);

            v[left..right].copy_from_slice(&buffer[left..right]);
        }

        width *= 2;
    }
}

fn merge<T: Ord + Copy + std::fmt::Display>(left_half: &[T], right_half: &[T], v: &mut [T]) {
    let (mut i, mut j, mut k) = (0, 0, 0);

    while i < left_half.len() && j < right_half.len() {
        if left_half[i] < right_half[j] {
            v[k] = left_half[i];
            i += 1;
        } else {
            v[k] = right_half[j];
            j += 1;
        }
        k += 1;
    }

    while i < left_half.len() {
        v[k] = left_half[i];
        i += 1;
        k += 1;
    }

    while j < right_half.len() {
        v[k] = right_half[j];
        j += 1;
        k += 1;
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
