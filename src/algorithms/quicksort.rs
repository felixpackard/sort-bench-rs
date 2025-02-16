pub fn sort<T: Ord + Copy>(v: &mut [T]) {
    if v.len() <= 1 {
        return;
    }

    let capacity = (v.len() as f64).log2().floor() as usize + 1;
    let mut stack: Vec<&mut [T]> = Vec::with_capacity(capacity);
    let mut current = v;

    loop {
        if current.len() > 1 {
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

    while let Some(v) = stack.pop() {
        if v.len() > 1 {
            let p = partition(v);
            let (left, right) = v.split_at_mut(p);
            stack.push(left);
            stack.push(&mut right[1..]);
        }
    }
}

fn partition<T: Ord + Copy>(v: &mut [T]) -> usize {
    let right = v.len() - 1;
    let mut i = 0;
    for j in 0..right {
        if v[j] <= v[right] {
            v.swap(j, i);
            i += 1;
        }
    }
    v.swap(i, right);
    i
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
