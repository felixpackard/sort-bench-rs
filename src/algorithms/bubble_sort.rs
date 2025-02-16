pub fn sort<T: Ord + Copy>(v: &mut [T]) {
    for i in 0..v.len() {
        let mut swapped = false;
        for j in 0..v.len() - i - 1 {
            if v[j] > v[j + 1] {
                v.swap(j, j + 1);
                swapped = true;
            }
        }

        if !swapped {
            break;
        }
    }
}
