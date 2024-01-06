use std::ops::Range;
use std::process;
use rayon::prelude::*;

const fn numbers_are_unique(numbers: &[usize; 9]) -> bool {
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

const fn sums_are_equal(numbers: &[usize; 9]) -> bool {
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

fn main() {
    const LIMIT: usize = 60;
    const NUMBER_ITERATOR: Range<usize> = 0..LIMIT;

    NUMBER_ITERATOR.for_each(|a| {
        NUMBER_ITERATOR.into_par_iter().for_each(|b| {
            NUMBER_ITERATOR.for_each(|c| {
                NUMBER_ITERATOR.for_each(|d| {
                    NUMBER_ITERATOR.for_each(|e| {
                        NUMBER_ITERATOR.for_each(|f| {
                            NUMBER_ITERATOR.for_each(|g| {
                                NUMBER_ITERATOR.for_each(|h| {
                                    NUMBER_ITERATOR.for_each(|i| {
                                        if numbers_are_unique(&[a, b, c, d, e, f, g, h, i]) && sums_are_equal(&[a * a, b * b, c * c, d * d, e * e, f * f, g * g, h * h, i * i]) {
                                            println!("{:?}", [a * a, b * b, c * c, d * d, e * e, f * f, g * g, h * h, i * i]);
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
        println!("{} / {}", a + 1, LIMIT);
    });
}