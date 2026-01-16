use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use std::f64;

pub fn gen_uni_rand(n: usize, seed: u64) -> Vec<i64> {
    let rng = SmallRng::seed_from_u64(seed as u64);
    rng.random_iter().take(n).collect()
}

pub fn gen_sorted(n: usize, seed: u64) -> Vec<i64> {
    let mut res = gen_uni_rand(n, seed);
    res.sort();
    res // This is increasing
}
pub fn gen_rev_sorted(n: usize, seed: u64) -> Vec<i64> {
    let mut res = gen_uni_rand(n, seed);
    res.sort();
    res.reverse();
    res // This is decreasing 
}
pub fn gen_almost_sorted(n: usize, seed: u64) -> Vec<i64> {
    let mut res = gen_sorted(n, seed);
    let mut rng = SmallRng::seed_from_u64(seed as u64);
    let num_swaps = f64::round(n as f64 / 4.0) as usize;
    for _ in 0..num_swaps {
        let place1: usize = rng.random_range(..n);
        let place2: usize = rng.random_range(..n);
        res.swap(place1, place2);
    }
    res
}
pub fn gen_organ_pipe(n: usize, seed: u64) -> Vec<i64> {
    let inc_bottem_half = f64::ceil(n as f64 / 2.0) as usize;
    let mut maybe_larger_half = gen_sorted(inc_bottem_half, seed);
    let mut maybe_smaller_half = gen_sorted(inc_bottem_half - 1, seed);
    if maybe_larger_half.iter().max() > maybe_smaller_half.iter().max() {
        maybe_larger_half.append(&mut maybe_smaller_half.drain(..).rev().collect());
        maybe_larger_half
    } else {
        maybe_smaller_half.append(&mut maybe_larger_half.drain(..).rev().collect());
        maybe_smaller_half
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_gen_uni_rand() {
        let n = 50;
        let seed = 42;
        let res: Vec<i64> = gen_uni_rand(n, seed); // checks the type 
        assert_eq!(res.len(), n); // checks the length
        // If this is ever sorted I should buy a lottery ticket
        assert!(!(res.iter().is_sorted()))
    }

    #[test]
    fn test_gen_sorted() {
        let n = 50;
        let seed = 42;
        let res: Vec<i64> = gen_sorted(n, seed); // checks the type 
        assert_eq!(res.len(), n); // checks the length
        assert!(res.iter().is_sorted()) // checks sorting
    }

    #[test]
    fn test_gen_rev_sorted() {
        let n = 50;
        let seed = 42;
        let res: Vec<i64> = gen_rev_sorted(n, seed); // checks the type 
        assert_eq!(res.len(), n); // checks the length
        assert!(res.iter().rev().is_sorted()) // checks reverse sorting
    }

    #[test]
    fn test_gen_almost_sorted() {
        let n = 50;
        let seed = 42;
        let res: Vec<i64> = gen_almost_sorted(n, seed); // checks the type 
        assert_eq!(res.len(), n); // checks the length
        // If this is ever sorted I should buy a lottery ticket
        assert!(!(res.iter().is_sorted()))
    }

    #[test]
    fn test_gen_organ_pipe() {
        let n = 5;
        let seed = 42;
        let inc_bottem_half = f64::ceil(n as f64 / 2.0) as usize;
        let res: Vec<i64> = gen_organ_pipe(n, seed); // checks the type 
        assert!(res[..inc_bottem_half].iter().is_sorted());
        assert!(res[inc_bottem_half..].iter().rev().is_sorted());
        assert!(res[..inc_bottem_half].iter().max() > res[inc_bottem_half..].iter().max());
        // test even case
        let n = 4;
        let seed = 42;
        let inc_bottem_half = f64::ceil(n as f64 / 2.0) as usize;
        let res: Vec<i64> = gen_organ_pipe(n, seed); // checks the type 
        assert!(res[..inc_bottem_half].iter().is_sorted());
        assert!(res[inc_bottem_half..].iter().rev().is_sorted());
        assert!(res[..inc_bottem_half].iter().max() > res[inc_bottem_half..].iter().max());
        // just to make sure
        for i in 1..30 {
            let n = i.clone() as usize;
            let seed = i.clone() as u64;
            let inc_bottem_half = f64::ceil(n as f64 / 2.0) as usize;
            let res: Vec<i64> = gen_organ_pipe(n, seed); // checks the type 
            // The next two cases check for the "pipe organ" structure of the data
            assert!(res[..inc_bottem_half].iter().is_sorted());
            assert!(res[inc_bottem_half..].iter().rev().is_sorted());
            assert!(res[..inc_bottem_half].iter().max() > res[inc_bottem_half..].iter().max());
            // The last case makes sure it is strictly increasing until half
        }
    }
}
