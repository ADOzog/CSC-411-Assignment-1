mod bubble;
use bubble::*;
mod insert;
use insert::*;
mod merge;
use merge::*;
mod input_gen;
use input_gen::*;

use serde::Serialize;
use std::{fs::File, io::Write, time::Instant};

#[derive(Clone, Serialize)]
pub enum SortingMethod {
    Merge,
    Insertion,
    Bubble,
}

#[derive(Clone, Serialize)]
pub enum InputDataType {
    UniRand,
    Sorted,
    RevSorted,
    AlmostSorted,
    PipeOrgan,
}

#[derive(Serialize)]
struct TrialResult {
    sorting_method: SortingMethod,
    input_data_type: InputDataType,
    given_n: usize,
    durations: Vec<usize>, // In
}

fn main() {
    const SEED: u64 = 42;
    // This is geometric growth with g_0 = 2 and a growth rate of 2.86, all the way to 10,000
    let ns: Vec<usize> = vec![2, 6, 16, 47, 135, 387, 1101, 3153, 9019, 10000];
    let methods: Vec<SortingMethod> = vec![
        SortingMethod::Merge,
        SortingMethod::Insertion,
        SortingMethod::Bubble,
    ];
    let input_types: Vec<InputDataType> = vec![
        InputDataType::UniRand,
        InputDataType::Sorted,
        InputDataType::RevSorted,
        InputDataType::AlmostSorted,
        InputDataType::PipeOrgan,
    ];
    let mut all_trial_results: Vec<TrialResult> = vec![];
    // There is a much better way to do this but this is the one part of my code where speed
    // does not matter
    for method in methods.clone() {
        for input_type in input_types.clone() {
            for n in ns.clone() {
                // This should have been borrows
                all_trial_results.push(run_indiv_trial(
                    n.clone(),
                    method.clone(),
                    input_type.clone(),
                    SEED,
                ));
            }
        }
    }

    write_vec_to_json(all_trial_results);
    println!("All done!");

    // For the degraded spacial locality

    let mut all_trial_results_deg_space: Vec<TrialResult> = vec![];
    // There is a much better way to do this but this is the one part of my code where speed
    // does not matter
    for method in methods.clone() {
        for input_type in input_types.clone() {
            for n in ns.clone() {
                // This should have been borrows
                all_trial_results_deg_space.push(run_indiv_trial_deg_space(
                    n.clone(),
                    method.clone(),
                    input_type.clone(),
                    SEED,
                ));
            }
        }
    }

    write_vec_to_json(all_trial_results_deg_space);
    println!("All done with EX part")
}

fn write_vec_to_json(data_vec: Vec<TrialResult>) {
    let json_data = serde_json::to_string(&data_vec).unwrap();
    let mut file = File::create("deg_space_results.json").unwrap();
    let res = file.write_all(json_data.as_bytes());
    res.unwrap()
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
        trial_times.push(start_time.elapsed().as_nanos() as usize);
    }
    return TrialResult {
        sorting_method: method,
        input_data_type: input_type,
        given_n: n,
        durations: trial_times,
    };
}
fn run_indiv_trial_deg_space(
    n: usize,
    method: SortingMethod,
    input_type: InputDataType,
    seed: u64,
) -> TrialResult {
    // gonna need match statements here

    let mut trial_times: Vec<usize> = Vec::with_capacity(30);
    let given_fn: fn(Vec<Ising>) -> Vec<Ising> = match method {
        SortingMethod::Merge => std_rec_merge_sort_sings,
        SortingMethod::Insertion => insertion_sort_sings,
        SortingMethod::Bubble => bubble_sort_sings,
    };
    for _ in 0..30 {
        let input_data = vec_to_vec_of_singletons(match input_type {
            InputDataType::UniRand => gen_uni_rand(n, seed),
            InputDataType::Sorted => gen_sorted(n, seed),
            InputDataType::RevSorted => gen_rev_sorted(n, seed),
            InputDataType::AlmostSorted => gen_almost_sorted(n, seed),
            InputDataType::PipeOrgan => gen_organ_pipe(n, seed),
        });
        let start_time = Instant::now();
        // call the fn here
        let _ = given_fn(input_data);
        trial_times.push(start_time.elapsed().as_nanos() as usize);
    }
    return TrialResult {
        sorting_method: method,
        input_data_type: input_type,
        given_n: n,
        durations: trial_times,
    };
}

#[cfg(test)]
mod main_tests {
    use crate::{
        InputDataType, SortingMethod, TrialResult, run_indiv_trial, run_indiv_trial_deg_space,
    };

    #[test]
    fn test_run_indiv_trial() {
        let seed = 42;
        let res: TrialResult =
            run_indiv_trial(2, SortingMethod::Bubble, InputDataType::UniRand, seed); // tests the type
        assert_eq!(res.durations.len(), 30);
    }
    #[test]
    fn test_run_indiv_trial_deg_space() {
        let seed = 42;
        let res: TrialResult =
            run_indiv_trial_deg_space(2, SortingMethod::Bubble, InputDataType::UniRand, seed); // tests the type
        assert_eq!(res.durations.len(), 30);
    }
}
