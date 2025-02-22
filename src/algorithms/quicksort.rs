pub fn sort<T: Ord + Copy>(v: &mut [T]) {
    if v.len() <= 1 {
        return;
    }

    let capacity = (v.len() as f64).log2().floor() as usize + 1;
    let mut stack: Vec<&mut [T]> = Vec::with_capacity(capacity);
    let mut current = v;

    loop {
        if current.len() < 20 {
            crate::utils::insertion_sort(current);
            match stack.pop() {
                Some(slice) => current = slice,
                None => break,
            };
        } else if current.len() > 1 {
            let p = partition(current);
            let (left, right) = current.split_at_mut(p + 1);

            // exclude the pivot from the right partition
            // let right = &mut right[1..];

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

fn pivot<T: Ord + Copy>(v: &mut [T]) -> T {
    let left = 0;
    let right = v.len() - 1;
    let mid = (left + right) / 2;
    if v[right] < v[left] {
        v.swap(left, right);
    }
    if v[mid] < v[left] {
        v.swap(left, mid);
    }
    if v[right] < v[mid] {
        v.swap(mid, right);
    }
    v[mid]
}

/// Hoare's partition scheme
fn partition<T: Ord + Copy>(v: &mut [T]) -> usize {
    let p = pivot(v);
    let mut left: isize = -1;
    let mut right: isize = v.len() as isize;

    loop {
        while {
            left += 1;
            v[left as usize] < p
        } {}

        while {
            right -= 1;
            v[right as usize] > p
        } {}

        if left >= right {
            return right as usize;
        }

        v.swap(left as usize, right as usize);
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
