use itertools::Itertools;

fn generate_square_numbers(count: i32) -> Vec<i32> {
    (0..=count).map(|number| number * number).collect()
}

fn rotate_vector(layout: Vec<i32>) -> Vec<i32> {
    vec!(
        layout[6], layout[3], layout[0],
        layout[7], layout[4], layout[1],
        layout[8], layout[5], layout[2],
    )
}

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

fn generate_permutations() -> Vec<Vec<i32>> {
    let numbers: Vec<i32> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];
    let mut result: Vec<Vec<i32>> = Vec::new();
    let mut stack: Vec<(i32, Vec<i32>)> = vec![(0, numbers.clone())];

    while let Some((index, mut remaining)) = stack.pop() {
        if index == 9 {
            result.push(remaining);
            continue;
        }

        for i in index..9 {
            remaining.swap(index as usize, i as usize);
            stack.push((index + 1, remaining.clone()));
        }
    }

    result.clone().iter().for_each(|permutation| {
        let rotated_90: Vec<i32> = rotate_vector(permutation.clone());
        let rotated_180: Vec<i32> = rotate_vector(rotated_90.clone());
        let rotated_270: Vec<i32> = rotate_vector(rotated_180.clone());

        if let Some(index) = result.iter().position(|value| value == &rotated_90) {
            result.remove(index);
        }

        if let Some(index) = result.iter().position(|value| value == &rotated_180) {
            result.remove(index);
        }

        if let Some(index) = result.iter().position(|value| value == &rotated_270) {
            result.remove(index);
        }
    });

    result
}

fn main() {
    const LIMIT: i32 = 9;

    let square_numbers: Vec<i32> = generate_square_numbers(LIMIT);

    // generate 9! permutations
    // then for every iteration, plug in a different combination of indices
    // for the squares vector

    let indices: Vec<Vec<i32>> = generate_permutations();

    println!("Generating combinations...");
    let combinations: itertools::Combinations<std::slice::Iter<'_, i32>> = square_numbers.iter().combinations(9);

    println!("Processing...");
    combinations.for_each(|combination: Vec<&i32>| {
        indices.clone().iter().for_each(|index| {
            let data: Vec<i32> = index.iter().map(|&i| combination[i as usize]).cloned().collect();

            if sums_are_equal(&data) {
                println!("{:?}", data);
            }
        });
    });
}
