pub fn sort<T: Ord + Copy>(v: &mut [T]) {
    if v.len() <= 1 {
        return;
    }

    let capacity = (v.len() as f64).log2().floor() as usize + 1;
    let mut stack: Vec<&mut [T]> = Vec::with_capacity(capacity);
    let mut current = v;

    loop {
        if current.len() < 10 {
            insertion_sort(current);
            match stack.pop() {
                Some(slice) => current = slice,
                None => break,
            };
        } else if current.len() > 1 {
            let p = partition(current);
            let (left, right) = current.split_at_mut(p);

            // exclude the pivot from the right partition
            let right = &mut right[1..];

            if left.len() < right.len() {
                stack.push(right);
                current = left;
            } else {
                stack.push(left);
                current = right;
            }
        } else if let Some(slice) = stack.pop() {
            current = slice;
        } else {
            break;
        }
    }
}

/// Hoare's partition scheme
fn partition<T: Ord + Copy>(v: &mut [T]) -> usize {
    let pivot = v[0];
    let mut left = 0;
    let mut right = v.len() - 1;

    loop {
        while v[left] < pivot {
            left += 1;
        }

        while v[right] > pivot {
            right -= 1;
        }

        if left >= right {
            return right;
        }

        v.swap(left, right);

        left += 1;
        right -= 1;
    }
}

fn insertion_sort<T: Ord + Copy>(v: &mut [T]) {
    for i in 1..v.len() {
        let mut j = i;
        let key = v[i];

        while j > 0 && key < v[j - 1] {
            v[j] = v[j - 1];
            j -= 1;
        }

        v[j] = key;
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_sort() {
        let mut data = crate::utils::rand_data(1_000_000);
        super::sort(&mut data);
        crate::utils::assert_sorted(&data);
    }
}
