fn merge_sort(unsorted_vec: Vec<i64>) -> Vec<i64> {
    todo!()
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_merge_sort() {
        assert_eq!(merge_sort(vec![5, 3, 2, 1, 4]), vec![1, 2, 3, 4, 5]);
    }
}
