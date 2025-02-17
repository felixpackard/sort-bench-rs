pub fn sort<T: Ord + Copy>(v: &mut [T]) {
    let n = v.len();
    if n <= 1 {
        return;
    }

    let mut start = n / 2;
    let mut end = n;
    while end > 1 {
        if start > 0 {
            start -= 1;
        } else {
            end -= 1;
            v.swap(0, end);
        }

        let mut root = start;
        while left_child_idx(root) < end {
            let mut child = left_child_idx(root);
            if child + 1 < end && v[child] < v[child + 1] {
                child += 1;
            }

            if v[root] < v[child] {
                v.swap(root, child);
                root = child;
            } else {
                break;
            }
        }
    }
}

fn left_child_idx(i: usize) -> usize {
    2 * i + 1
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
