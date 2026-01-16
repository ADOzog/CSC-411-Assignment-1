pub fn insertion_sort(mut input_vec: Vec<i64>) -> Vec<i64> {
    let mut max_pos_checked: usize = 1;
    let len_vec = input_vec.len();
    while max_pos_checked < len_vec {
        let mut cur_pos = max_pos_checked.clone();
        while cur_pos > 0 && input_vec[cur_pos - 1] > input_vec[cur_pos] {
            input_vec.swap(cur_pos, cur_pos - 1);
            cur_pos -= 1;
        }
        max_pos_checked += 1
    }
    input_vec
}
#[cfg(test)]
mod test {
    use super::super::input_gen::gen_uni_rand;
    use super::*;
    #[test]
    fn test_insertion_sort() {
        assert_eq!(insertion_sort(vec![5, 3, 2, 1, 4]), vec![1, 2, 3, 4, 5]);
        assert_eq!(insertion_sort(vec![1; 100]), vec![1; 100]);
        let mut rvec = gen_uni_rand(100, 42);
        let res = insertion_sort(rvec.clone());
        rvec.sort();
        assert_eq!(res, rvec)
    }
}
