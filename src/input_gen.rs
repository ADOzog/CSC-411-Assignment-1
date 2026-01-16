use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use std::vec;

fn gen_uni_rand(n: usize, seed: u64) -> Vec<i64> {
    let rng = SmallRng::seed_from_u64(seed as u64);
    rng.random_iter().take(n).collect()
}

fn gen_sorted(n: usize, seed: u64) -> Vec<i64> {
    let mut res = gen_uni_rand(n, seed);
    res.sort();
    res // This is increasing
}
fn gen_rev_sorted(n: usize, seed: u64) -> Vec<i64> {
    let mut res = gen_uni_rand(n, seed);
    res.sort();
    res.reverse();
    res // This is decreasing 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gen_uni_rand() {
        let n = 5;
        let seed = 42;
        let res: Vec<i64> = gen_uni_rand(n, seed); // checks the type 
        assert_eq!(res.len(), n);
    }

    #[test]
    fn test_gen_sorted() {
        let n = 5;
        let seed = 42;
        let res: Vec<i64> = gen_sorted(n, seed); // checks the type 
        assert_eq!(res.len(), n);
    }

    #[test]
    fn test_gen_rev_sorted() {
        let n = 5;
        let seed = 42;
        let res: Vec<i64> = gen_rev_sorted(n, seed); // checks the type 
        assert_eq!(res.len(), n);
    }
}
