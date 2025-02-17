pub fn sort<T: Ord + Copy>(v: &mut [T]) {
    let n = v.len();
    if n <= 1 {
        return;
    }

    let mut width = 1;
    while width < n {
        let mut left = 0;
        while left < n {
            let right = std::cmp::min(left + width * 2, n);
            merge(&mut v[left..right]);
            left += width * 2;
        }
        width *= 2;
    }
}

fn merge<T: Ord + Copy>(v: &mut [T]) {
    let mid = v.len() / 2;
    let left = v[..mid].to_vec();
    let right = v[mid..].to_vec();

    let (mut i, mut j, mut k) = (0, 0, 0);
    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            v[k] = left[i];
            i += 1;
        } else {
            v[k] = right[j];
            j += 1;
        }
        k += 1;
    }

    while i < left.len() {
        v[k] = left[i];
        i += 1;
        k += 1;
    }

    while j < right.len() {
        v[k] = right[j];
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
