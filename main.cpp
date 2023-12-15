#include <iostream>
#include <vector>

bool sumsAreEqual(const std::vector<int32_t>& numbers) {
    const int32_t top_row_sum = numbers[0] + numbers[1] + numbers[2];
    const int32_t middle_row_sum = numbers[3] + numbers[4] + numbers[5];
    const int32_t bottom_row_sum = numbers[6] + numbers[7] + numbers[8];

    if (top_row_sum != middle_row_sum or middle_row_sum != bottom_row_sum) {
        return false;
    }

    const int32_t left_column_sum = numbers[0] + numbers[3] + numbers[6];
    const int32_t middle_column_sum = numbers[1] + numbers[4] + numbers[7];
    const int32_t right_column_sum = numbers[2] + numbers[5] + numbers[8];

    if (bottom_row_sum != left_column_sum or left_column_sum != middle_column_sum or middle_column_sum != right_column_sum) {
        return false;
    }

    const int32_t nw_se_sum = numbers[0] + numbers[4] + numbers[8];
    const int32_t sw_ne_sum = numbers[6] + numbers[4] + numbers[2];

    return right_column_sum == nw_se_sum and nw_se_sum == sw_ne_sum;
}

// def main():
//     limit: int = 20
//
//     let square_numbers: Vec<int> = (0..=limit).map(|i| i * i).collect();
//
//     // generate 9! permutations
//     // then for every iteration, plug in a different combination of indices
//     // for the squares vector
//
//     let unfiltered_indices: Vec<Vec<int>> = (0..9)
//         .permutations(9)
//         .map(|permutation| permutation.into_iter().collect())
//         .collect();
//
//     let permutations_to_ignore: [[int; 9]; 7] = [
//         // rotation
//         [6, 3, 0, 7, 4, 1, 8, 5, 2],
//         [8, 7, 6, 5, 4, 3, 2, 1, 0],
//         [2, 5, 8, 1, 4, 7, 0, 3, 6],
//         // flip nw-se
//         [0, 3, 6, 1, 4, 7, 2, 5, 8],
//         // flip sw-ne
//         [8, 5, 2, 7, 4, 1, 6, 3, 0],
//         // flip n-s
//         [2, 1, 0, 5, 4, 3, 8, 7, 6],
//         // flip w-e
//         [6, 7, 8, 3, 4, 5, 0, 1, 2],
//     ];
//
//     // for every unfiltered_index in unfiltered_indices, remove any permutations that match the permutations_to_ignore array
//     let indices: Vec<Vec<int>> = unfiltered_indices
//         .iter()
//         .filter(|indices| {
//             permutations_to_ignore.iter().all(|permutation_to_ignore| {
//                 indices
//                     .iter()
//                     .zip(permutation_to_ignore.iter())
//                     .all(|(index, permutation_index)| index != permutation_index)
//             })
//         })
//         .cloned()
//         .collect();
//
//     let combinations: itertools::Combinations<std::slice::Iter<'_, int>> =
//         square_numbers.iter().combinations(9);
//
//     combinations
//         .par_bridge()
//         .for_each(|combination: Vec<&int>| {
//             indices.iter().for_each(|index| {
//                 let numbers: Vec<&int> = index.iter().map(|&i| combination[i as usize]).collect();
//
//                 if sums_are_equal(&numbers) {
//                     println!("{:?}", numbers);
//                 }
//             });
//         });

int main() {
    std::cout << "Hello, World!" << std::endl;
    return 0;
}
