use rayon::prelude::*;
use crate::sums_are_equal::sums_are_equal;

pub fn run_avx2(combinations: itertools::Combinations<std::slice::Iter<'_, i32>>, indices: Vec<Vec<i32>>) {
    combinations.par_bridge().for_each(|combination: Vec<&i32>| {
        indices.iter().for_each(|index| {
            let numbers: Vec<&i32> = index.iter().map(|&i| combination[i as usize]).collect();

            if sums_are_equal(&numbers) {
                println!("{:?}", numbers);
            }
        });
    });
}

pub fn run(combinations: itertools::Combinations<std::slice::Iter<'_, i32>>, indices: Vec<Vec<i32>>) {
    combinations.par_bridge().for_each(|combination: Vec<&i32>| {
        indices.iter().for_each(|index| {
            let numbers: Vec<&i32> = index.iter().map(|&i| combination[i as usize]).collect();

            if sums_are_equal(&numbers) {
                println!("{:?}", numbers);
            }
        });
    });
}