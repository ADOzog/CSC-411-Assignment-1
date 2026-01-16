mod bubble;
use bubble::*;
mod insert;
use insert::*;
mod merge;
use merge::*;
mod input_gen;
use input_gen::*;

use std::time::Instant;

pub enum SortingMethod {
    Merge,
    Insertion,
    Bubble,
}

pub enum InputDataType {
    UniRand,
    Sorted,
    RevSorted,
    AlmostSorted,
    PipeOrgan,
}

struct TrialResult {
    sorting_method: SortingMethod,
    input_data_type: InputDataType,
    given_n: usize,
    durations: Vec<usize>,
}

fn main() {
    // This is geometric growth with g_0 = 2 and a growth rate of 2.86, all the way to 50,000
    let ns: Vec<usize> = vec![2, 6, 16, 47, 133, 378, 1084, 3103, 8850, 25380, 50000];
}

fn run_indiv_trial(
    n: usize,
    method: SortingMethod,
    input_type: InputDataType,
    seed: u64,
) -> TrialResult {
    // gonna need match statements here

    let mut trial_times: Vec<usize> = Vec::with_capacity(30);
    let given_fn: fn(Vec<i64>) -> Vec<i64> = match method {
        SortingMethod::Merge => std_rec_merge_sort,
        SortingMethod::Insertion => insertion_sort,
        SortingMethod::Bubble => bubble_sort,
    };
    for _ in 0..30 {
        let input_data = match input_type {
            InputDataType::UniRand => gen_uni_rand(n, seed),
            InputDataType::Sorted => gen_sorted(n, seed),
            InputDataType::RevSorted => gen_rev_sorted(n, seed),
            InputDataType::AlmostSorted => gen_almost_sorted(n, seed),
            InputDataType::PipeOrgan => gen_organ_pipe(n, seed),
        };
        let start_time = Instant::now();
        // call the fn here
        let _ = given_fn(input_data);
        trial_times.push(start_time.elapsed().as_millis() as usize);
    }
    return TrialResult {
        sorting_method: method,
        input_data_type: input_type,
        given_n: n,
        durations: trial_times,
    };
}

#[cfg(test)]
mod tests {
    use crate::{InputDataType, SortingMethod, TrialResult, run_indiv_trial};

    #[test]
    fn test_run_indiv_trial() {
        let seed = 42;
        let res: TrialResult =
            run_indiv_trial(2, SortingMethod::Bubble, InputDataType::UniRand, seed); // tests the type
        assert_eq!(res.durations.len(), 30);
    }
}
