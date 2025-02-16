pub fn sort<T: Ord + Copy>(v: &mut [T]) {
    if v.len() <= 1 {
        return;
    }

    for left in 0..v.len() {
        let mut idx = left;
        for right in left + 1..v.len() {
            if v[right] < v[idx] {
                idx = right;
            }
        }
        v.swap(left, idx);
    }
}
