mod sums_are_equal;
mod run;

use itertools::Itertools;
use crate::run::{run_with_avx2, run_without_avx2};

fn main() {
    const LIMIT: i32 = 17;

    let square_numbers: Vec<i32> = (0..=LIMIT).map(|i| i * i).collect();

    // generate 9! permutations
    // then for every iteration, plug in a different combination of indices
    // for the squares vector

    let unfiltered_indices: Vec<Vec<i32>> = (0..9).permutations(9).map(|permutation| permutation.into_iter().collect()).collect();

    let permutations_to_ignore: [[i32; 9]; 7] = [
        // rotation
        [6, 3, 0, 7, 4, 1, 8, 5, 2],
        [8, 7, 6, 5, 4, 3, 2, 1, 0],
        [2, 5, 8, 1, 4, 7, 0, 3, 6],

        // flip nw-se
        [0, 3, 6, 1, 4, 7, 2, 5, 8],

        // flip sw-ne
        [8, 5, 2, 7, 4, 1, 6, 3, 0],

        // flip n-s
        [2, 1, 0, 5, 4, 3, 8, 7, 6],

        // flip w-e
        [6, 7, 8, 3, 4, 5, 0, 1, 2],
    ];

    // for every unfiltered_index in unfiltered_indices, remove any permutations that match the permutations_to_ignore array
    let indices: Vec<Vec<i32>> = unfiltered_indices
        .iter()
        .filter(|indices| {
            permutations_to_ignore
                .iter()
                .all(|permutation_to_ignore| {
                    indices
                      .iter()
                      .zip(permutation_to_ignore.iter())
                      .all(|(index, permutation_index)| index != permutation_index)
                })
        })
        .cloned()
        .collect();

    let combinations: itertools::Combinations<std::slice::Iter<'_, i32>> = square_numbers.iter().combinations(9);

    if is_x86_feature_detected!("avx2") {
        println!("Use AVX2 SIMD instructions");

        run_with_avx2(combinations, indices);
    } else {
        println!("Don't use AVX2 SIMD instructions");

        run_without_avx2(combinations, indices);
    }
}
