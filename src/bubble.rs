use crate::input_gen::Ising;

pub fn bubble_sort(mut input_vec: Vec<i64>) -> Vec<i64> {
    let mut did_swap = true; // starts as true to enter loop
    let vec_len = input_vec.len() - 1;
    // this does exit early
    while did_swap {
        did_swap = false;
        for i in 0..vec_len {
            match (input_vec[i] > input_vec[i + 1], did_swap) {
                (true, false) => {
                    did_swap = true;
                    input_vec.swap(i, i + 1);
                }
                (true, true) => {
                    input_vec.swap(i, i + 1);
                }
                (_, _) => continue,
            }
        }
    }
    input_vec
}
pub fn bubble_sort_sings(mut input_vec: Vec<Ising>) -> Vec<Ising> {
    let mut did_swap = true; // starts as true to enter loop
    let vec_len = input_vec.len() - 1;
    // this does exit early
    while did_swap {
        did_swap = false;
        for i in 0..vec_len {
            match (input_vec[i].data > input_vec[i + 1].data, did_swap) {
                (true, false) => {
                    did_swap = true;
                    input_vec.swap(i, i + 1);
                }
                (true, true) => {
                    input_vec.swap(i, i + 1);
                }
                (_, _) => continue,
            }
        }
    }
    input_vec
}
#[cfg(test)]
mod test {
    use super::super::input_gen::{gen_uni_rand, vec_to_vec_of_singletons};
    use super::*;
    #[test]
    fn test_bubble_sort() {
        assert_eq!(bubble_sort(vec![5, 3, 2, 1, 4]), vec![1, 2, 3, 4, 5]);
        assert_eq!(bubble_sort(vec![1; 50]), vec![1; 50]);
        assert_eq!(bubble_sort(vec![0]), vec![0]);
        //
        let mut rvec = gen_uni_rand(100, 42);
        let res = bubble_sort(rvec.clone());
        rvec.sort();
        assert_eq!(res, rvec);
        // How should I test for early exit
    }
    #[test]
    fn test_bubble_sort_sings() {
        let mut rvec = gen_uni_rand(100, 42);
        let res = bubble_sort_sings(vec_to_vec_of_singletons(rvec.clone()));
        rvec.sort();
        assert_eq!(res, vec_to_vec_of_singletons(rvec));
        // How should I test for early exit
    }
}
