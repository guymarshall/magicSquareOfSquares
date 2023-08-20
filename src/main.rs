use itertools::Itertools;

fn sums_are_equal(numbers: &Vec<i32>) -> bool {
    let top_row_sum: i32 = numbers[0] + numbers[1] + numbers[2];
    let middle_row_sum: i32 = numbers[3] + numbers[4] + numbers[5];
    let bottom_row_sum: i32 = numbers[6] + numbers[7] + numbers[8];

    if top_row_sum != middle_row_sum || middle_row_sum != bottom_row_sum {
        return false;
    }

    let left_column_sum: i32 = numbers[0] + numbers[3] + numbers[6];
    let middle_column_sum: i32 = numbers[1] + numbers[4] + numbers[7];
    let right_column_sum: i32 = numbers[2] + numbers[5] + numbers[8];

    if bottom_row_sum != left_column_sum || left_column_sum != middle_column_sum || middle_column_sum != right_column_sum {
        return false;
    }

    let nw_se_sum: i32 = numbers[0] + numbers[4] + numbers[8];
    let sw_ne_sum: i32 = numbers[6] + numbers[4] + numbers[2];

    right_column_sum == nw_se_sum && nw_se_sum == sw_ne_sum
}

fn main() {
    const LIMIT: i32 = 9;

    let square_numbers: Vec<i32> = (0..=LIMIT).map(|i| i * i).collect();

    // generate 9! permutations
    // then for every iteration, plug in a different combination of indices
    // for the squares vector

    let indices: Vec<Vec<i32>> = (0..9).permutations(9).map(|permutation| permutation.into_iter().collect()).collect();
    println!("Permutation count: {}", indices.len());

    // go through indexes such as flipped and for each flipped rotated vector and loop through them all after generating them

    // let permutations_of_indexes_to_ignore: [[i32; 9]; 19] = [
    //     // just rotation
    //     [6, 3, 0, 7, 4, 1, 8, 5, 2],
    //     [8, 7, 6, 5, 4, 3, 2, 1, 0],
    //     [2, 5, 8, 1, 4, 7, 0, 3, 6],
    //
    //     // just flip nw-se
    //     [0, 3, 6, 1, 4, 7, 2, 5, 8],
    //
    //     // rotations of flip nw-se
    //     [, , , , , , , , ],
    //     [, , , , , , , , ],
    //     [, , , , , , , , ],
    //
    //     // just flip sw-ne
    //     [8, 5, 2, 7, 4, 1, 6, 3, 0],
    //
    //     // rotations of flip sw-ne
    //     [, , , , , , , , ],
    //     [, , , , , , , , ],
    //     [, , , , , , , , ],
    //
    //     // just flip n-s
    //     [2, 1, 0, 5, 4, 3, 8, 7, 6],
    //
    //     // rotations of flip n-s
    //     [, , , , , , , , ],
    //     [, , , , , , , , ],
    //     [, , , , , , , , ],
    //
    //     // just flip w-e
    //     [6, 7, 8, 3, 4, 5, 0, 1, 2],
    //
    //     // rotations of flip w-e
    //     [, , , , , , , , ],
    //     [, , , , , , , , ],
    //     [, , , , , , , , ],
    // ];

    println!("Generating combinations...");
    let combinations: itertools::Combinations<std::slice::Iter<'_, i32>> = square_numbers.iter().combinations(9);

    println!("Processing...");
    combinations.for_each(|combination: Vec<&i32>| {
        indices.clone().iter().for_each(|index| {
            let numbers: Vec<i32> = index.iter().map(|&i| combination[i as usize]).cloned().collect();

            if sums_are_equal(&numbers) {
                println!("{:?}", numbers);
            }
        });
    });
}
