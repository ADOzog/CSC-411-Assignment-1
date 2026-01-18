use crate::input_gen::Ising;

pub fn std_rec_merge_sort(input_vec: Vec<i64>) -> Vec<i64> {
    let len_vec = input_vec.len();
    if len_vec <= 1 {
        input_vec
    } else {
        let half_way = f64::ceil(len_vec as f64 / 2.0) as usize;
        // The unsafe block ignores the bounds check
        let (left, right) = unsafe { input_vec.split_at_unchecked(half_way) };
        let s_left = std_rec_merge_sort(left.to_vec());
        let s_right = std_rec_merge_sort(right.to_vec());
        merge(s_left, s_right)
    }
}
fn merge(in_vec_1: Vec<i64>, in_vec_2: Vec<i64>) -> Vec<i64> {
    let mut res_vec: Vec<i64> = Vec::with_capacity(in_vec_1.len() + in_vec_2.len());
    let mut vec_1 = in_vec_1.into_iter();
    let mut vec_2 = in_vec_2.into_iter();
    let mut dr = vec_1.next();
    let mut dl = vec_2.next();
    while dl.is_some() || dr.is_some() {
        match (dl, dr) {
            (Some(l), Some(r)) => {
                if l >= r {
                    res_vec.push(r);
                    dr = vec_1.next();
                } else {
                    res_vec.push(l);
                    dl = vec_2.next();
                }
            }
            (None, Some(r)) => {
                res_vec.push(r);
                dr = vec_1.next()
            }
            (Some(l), None) => {
                res_vec.push(l);
                dl = vec_2.next()
            }
            (None, None) => unreachable!(),
        }
    }
    res_vec
}
pub fn std_rec_merge_sort_sings(input_vec: Vec<Ising>) -> Vec<Ising> {
    let len_vec = input_vec.len();
    if len_vec <= 1 {
        input_vec
    } else {
        let half_way = f64::ceil(len_vec as f64 / 2.0) as usize;
        // The unsafe block ignores the bounds check
        let (left, right) = unsafe { input_vec.split_at_unchecked(half_way) };
        let s_left = std_rec_merge_sort_sings(left.to_vec());
        let s_right = std_rec_merge_sort_sings(right.to_vec());
        merge_sings(s_left, s_right)
    }
}
fn merge_sings(in_vec_1: Vec<Ising>, in_vec_2: Vec<Ising>) -> Vec<Ising> {
    let mut res_vec: Vec<Ising> = Vec::with_capacity(in_vec_1.len() + in_vec_2.len());
    let mut vec_1 = in_vec_1.into_iter();
    let mut vec_2 = in_vec_2.into_iter();
    let mut dr = vec_1.next();
    let mut dl = vec_2.next();
    while dl.is_some() || dr.is_some() {
        match (dl, dr) {
            (Some(l), Some(r)) => {
                if l.data >= r.data {
                    res_vec.push(r);
                    dr = vec_1.next();
                } else {
                    res_vec.push(l);
                    dl = vec_2.next();
                }
            }
            (None, Some(r)) => {
                res_vec.push(r);
                dr = vec_1.next()
            }
            (Some(l), None) => {
                res_vec.push(l);
                dl = vec_2.next()
            }
            (None, None) => unreachable!(),
        }
    }
    res_vec
}
#[cfg(test)]
mod test {
    use super::super::input_gen::{gen_uni_rand, vec_to_vec_of_singletons};
    use super::*;
    #[test]
    fn test_merge() {
        assert_eq!(merge(vec![1], vec![4]), vec![1, 4]);
        assert_eq!(merge(vec![1, 2], vec![4, 7]), vec![1, 2, 4, 7]);
    }
    #[test]
    fn test_merge_sort() {
        assert_eq!(std_rec_merge_sort(vec![5, 3, 2, 1, 4]), vec![1, 2, 3, 4, 5]);
        assert_eq!(std_rec_merge_sort(vec![1; 100]), vec![1; 100]);
        let mut rvec = gen_uni_rand(100, 42);
        let res = std_rec_merge_sort(rvec.clone());
        rvec.sort();
        assert_eq!(res, rvec)
    }
    #[test]
    fn test_merge_sort_sings() {
        let mut rvec = gen_uni_rand(100, 42);
        let res = std_rec_merge_sort_sings(vec_to_vec_of_singletons(rvec.clone()));
        rvec.sort();
        assert_eq!(res, vec_to_vec_of_singletons(rvec));
        // How should I test for early exit
    }
}
