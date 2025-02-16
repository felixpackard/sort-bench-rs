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

#[cfg(test)]
mod tests {
    #[test]
    fn test_sort() {
        let mut data = crate::utils::rand_data(10_000);
        super::sort(&mut data);
        crate::utils::assert_sorted(&data);
    }
}
