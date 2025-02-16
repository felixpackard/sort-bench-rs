pub fn sort<T: Ord + Copy>(v: &mut [T]) {
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
        let mut data = crate::utils::rand_data(10_000);
        super::sort(&mut data);
        crate::utils::assert_sorted(&data);
    }
}
