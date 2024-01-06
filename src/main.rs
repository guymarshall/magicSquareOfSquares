use std::process;
use rayon::prelude::*;

fn numbers_are_unique(numbers: &[&usize; 9]) -> bool {
    !(numbers[0] == numbers[1]
        || numbers[0] == numbers[2]
        || numbers[0] == numbers[3]
        || numbers[0] == numbers[4]
        || numbers[0] == numbers[5]
        || numbers[0] == numbers[6]
        || numbers[0] == numbers[7]
        || numbers[0] == numbers[8]
        || numbers[1] == numbers[2]
        || numbers[1] == numbers[3]
        || numbers[1] == numbers[4]
        || numbers[1] == numbers[5]
        || numbers[1] == numbers[6]
        || numbers[1] == numbers[7]
        || numbers[1] == numbers[8]
        || numbers[2] == numbers[3]
        || numbers[2] == numbers[4]
        || numbers[2] == numbers[5]
        || numbers[2] == numbers[6]
        || numbers[2] == numbers[7]
        || numbers[2] == numbers[8]
        || numbers[3] == numbers[4]
        || numbers[3] == numbers[5]
        || numbers[3] == numbers[6]
        || numbers[3] == numbers[7]
        || numbers[3] == numbers[8]
        || numbers[4] == numbers[5]
        || numbers[4] == numbers[6]
        || numbers[4] == numbers[7]
        || numbers[4] == numbers[8]
        || numbers[5] == numbers[6]
        || numbers[5] == numbers[7]
        || numbers[5] == numbers[8]
        || numbers[6] == numbers[7]
        || numbers[6] == numbers[8]
        || numbers[7] == numbers[8])
}

fn sums_are_equal(numbers: &[&usize; 9]) -> bool {
    let top_row_sum: usize = numbers[0] + numbers[1] + numbers[2];
    let middle_row_sum: usize = numbers[3] + numbers[4] + numbers[5];
    let bottom_row_sum: usize = numbers[6] + numbers[7] + numbers[8];

    if top_row_sum != middle_row_sum || middle_row_sum != bottom_row_sum {
        return false;
    }

    let left_column_sum: usize = numbers[0] + numbers[3] + numbers[6];
    let middle_column_sum: usize = numbers[1] + numbers[4] + numbers[7];
    let right_column_sum: usize = numbers[2] + numbers[5] + numbers[8];

    if bottom_row_sum != left_column_sum || left_column_sum != middle_column_sum || middle_column_sum != right_column_sum {
        return false;
    }

    let nw_se_sum: usize = numbers[0] + numbers[4] + numbers[8];
    let sw_ne_sum: usize = numbers[6] + numbers[4] + numbers[2];

    right_column_sum == nw_se_sum && nw_se_sum == sw_ne_sum
}

const fn generate_square_numbers<const COUNT: usize>() -> [usize; COUNT] {
    let mut numbers: [usize; COUNT] = [0usize; COUNT];

    let mut counter: usize = 0;
    while counter < COUNT {
        numbers[counter] = (counter + 1) * (counter + 1);
        counter += 1;
    }

    numbers
}

fn main() {
    const LIMIT: usize = 60;
    const LIMIT_SQUARED: usize = LIMIT * LIMIT;
    const SQUARE_NUMBERS: [usize; LIMIT] = generate_square_numbers();

    SQUARE_NUMBERS.iter().for_each(|a| {
        SQUARE_NUMBERS.par_iter().for_each(|b| {
            SQUARE_NUMBERS.iter().for_each(|c| {
                SQUARE_NUMBERS.iter().for_each(|d| {
                    SQUARE_NUMBERS.iter().for_each(|e| {
                        SQUARE_NUMBERS.iter().for_each(|f| {
                            SQUARE_NUMBERS.iter().for_each(|g| {
                                SQUARE_NUMBERS.iter().for_each(|h| {
                                    SQUARE_NUMBERS.iter().for_each(|i| {
                                        if numbers_are_unique(&[a, b, c, d, e, f, g, h, i]) && sums_are_equal(&[a, b, c, d, e, f, g, h, i]) {
                                            println!("{:?}", [a, b, c, d, e, f, g, h, i]);
                                            process::exit(0);
                                        }
                                    });
                                });
                            });
                        });
                    });
                });
            });
        });
        println!("{} / {}", a, LIMIT_SQUARED);
    });
}