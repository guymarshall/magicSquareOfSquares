import numpy as np


# Function to rotate a matrix using NumPy
def rotate_matrix_numpy(matrix):
    arr = np.array(matrix)
    rotated_matrix = np.rot90(arr, k=-1)  # Rotate 90 degrees counter-clockwise
    return rotated_matrix


# Function to filter elements in a matrix based on a condition
def filter_matrix(matrix, condition):
    arr = np.array(matrix)
    filtered_matrix = arr[condition]
    return filtered_matrix


# Example 3x3 matrix
matrix_3x3 = [
    [1, 2, 3],
    [4, 5, 6],
    [7, 8, 9]
]

# Rotate the matrix
rotated_90_np = rotate_matrix_numpy(matrix_3x3)

print("Rotated 90 degrees using NumPy:")
print(rotated_90_np)

# Filter the rotated matrix, for example, keeping elements greater than 5
filtered_result = filter_matrix(rotated_90_np, rotated_90_np > 5)

print("Filtered elements greater than 5:")
print(filtered_result)

# mod sums_are_equal;
# use crate::sums_are_equal::sums_are_equal;
#
# use itertools::Itertools;
# use rayon::prelude::*;
#
# pub fn sums_are_equal(numbers: &[&i32]) -> bool {
#     let top_row_sum: i32 = numbers[0] + numbers[1] + numbers[2];
#     let middle_row_sum: i32 = numbers[3] + numbers[4] + numbers[5];
#     let bottom_row_sum: i32 = numbers[6] + numbers[7] + numbers[8];
#
#     if top_row_sum != middle_row_sum || middle_row_sum != bottom_row_sum {
#         return false;
#     }
#
#     let left_column_sum: i32 = numbers[0] + numbers[3] + numbers[6];
#     let middle_column_sum: i32 = numbers[1] + numbers[4] + numbers[7];
#     let right_column_sum: i32 = numbers[2] + numbers[5] + numbers[8];
#
#     if bottom_row_sum != left_column_sum
#         || left_column_sum != middle_column_sum
#         || middle_column_sum != right_column_sum
#     {
#         return false;
#     }
#
#     let nw_se_sum: i32 = numbers[0] + numbers[4] + numbers[8];
#     let sw_ne_sum: i32 = numbers[6] + numbers[4] + numbers[2];
#
#     right_column_sum == nw_se_sum && nw_se_sum == sw_ne_sum
# }
#
#
# fn main() {
#     const LIMIT: i32 = 20;
#
#     let square_numbers: Vec<i32> = (0..=LIMIT).map(|i| i * i).collect();
#
#     // generate 9! permutations
#     // then for every iteration, plug in a different combination of indices
#     // for the squares vector
#
#     let unfiltered_indices: Vec<Vec<i32>> = (0..9)
#         .permutations(9)
#         .map(|permutation| permutation.into_iter().collect())
#         .collect();
#
#     let permutations_to_ignore: [[i32; 9]; 7] = [
#         // rotation
#         [6, 3, 0, 7, 4, 1, 8, 5, 2],
#         [8, 7, 6, 5, 4, 3, 2, 1, 0],
#         [2, 5, 8, 1, 4, 7, 0, 3, 6],
#         // flip nw-se
#         [0, 3, 6, 1, 4, 7, 2, 5, 8],
#         // flip sw-ne
#         [8, 5, 2, 7, 4, 1, 6, 3, 0],
#         // flip n-s
#         [2, 1, 0, 5, 4, 3, 8, 7, 6],
#         // flip w-e
#         [6, 7, 8, 3, 4, 5, 0, 1, 2],
#     ];
#
#     // for every unfiltered_index in unfiltered_indices, remove any permutations that match the permutations_to_ignore array
#     let indices: Vec<Vec<i32>> = unfiltered_indices
#         .iter()
#         .filter(|indices| {
#             permutations_to_ignore.iter().all(|permutation_to_ignore| {
#                 indices
#                     .iter()
#                     .zip(permutation_to_ignore.iter())
#                     .all(|(index, permutation_index)| index != permutation_index)
#             })
#         })
#         .cloned()
#         .collect();
#
#     let combinations: itertools::Combinations<std::slice::Iter<'_, i32>> =
#         square_numbers.iter().combinations(9);
#
#     combinations
#         .par_bridge()
#         .for_each(|combination: Vec<&i32>| {
#             indices.iter().for_each(|index| {
#                 let numbers: Vec<&i32> = index.iter().map(|&i| combination[i as usize]).collect();
#
#                 if sums_are_equal(&numbers) {
#                     println!("{:?}", numbers);
#                 }
#             });
#         });
# }
