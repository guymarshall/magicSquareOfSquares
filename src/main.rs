use std::process;
use crate::magic_square::MagicSquare;
use rayon::prelude::*;

mod magic_square;

fn numbers_are_unique(numbers: &[i32; 9]) -> bool {
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

fn sums_are_equal(numbers: &[i32; 9]) -> bool {
    let top_row_sum = numbers[0] + numbers[1] + numbers[2];
    let middle_row_sum = numbers[3] + numbers[4] + numbers[5];
    let bottom_row_sum = numbers[6] + numbers[7] + numbers[8];

    if top_row_sum != middle_row_sum || middle_row_sum != bottom_row_sum {
        return false;
    }

    let left_column_sum = numbers[0] + numbers[3] + numbers[6];
    let middle_column_sum = numbers[1] + numbers[4] + numbers[7];
    let right_column_sum = numbers[2] + numbers[5] + numbers[8];

    if bottom_row_sum != left_column_sum || left_column_sum != middle_column_sum || middle_column_sum != right_column_sum {
        return false;
    }

    let nw_se_sum = numbers[0] + numbers[4] + numbers[8];
    let sw_ne_sum = numbers[6] + numbers[4] + numbers[2];

    right_column_sum == nw_se_sum && nw_se_sum == sw_ne_sum
}

fn main() {
    const LIMIT: i32 = 50;

    (0..LIMIT).for_each(|a| {
        (0..LIMIT).into_par_iter().for_each(|b| {
            (0..LIMIT).for_each(|c| {
                (0..LIMIT).for_each(|d| {
                    (0..LIMIT).for_each(|e| {
                        (0..LIMIT).for_each(|f| {
                            (0..LIMIT).for_each(|g| {
                                (0..LIMIT).for_each(|h| {
                                    (0..LIMIT).for_each(|i| {
                                        if numbers_are_unique(&[a, b, c, d, e, f, g, h, i]) {
                                            let square: MagicSquare = MagicSquare::new(a, b, c, d, e, f, g, h, i);
                                            let is_magic: bool = square.sums_are_equal();

                                            if is_magic {
                                                println!("{:?}", square);
                                                process::exit(0);
                                            }
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
