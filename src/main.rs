use std::process;
use crate::magic_square::MagicSquare;
use rayon::prelude::*;

mod magic_square;

fn numbers_are_unique(numbers: &[u32; 9]) -> bool {
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

fn main() {
    const LIMIT: u32 = 50;

    (0..LIMIT).for_each(|a| {
        (0..LIMIT).into_par_iter().for_each(|b| {
            (0..LIMIT).for_each(|c| {
                (0..LIMIT).for_each(|d| {
                    (0..LIMIT).for_each(|e| {
                        (0..LIMIT).for_each(|f| {
                            (0..LIMIT).for_each(|g| {
                                (0..LIMIT).for_each(|h| {
                                    (0..LIMIT).for_each(|i| {
                                        let square_of_squares: [u32; 9] = [a, b, c, d, e, f, g, h, i];
                                        if numbers_are_unique(&square_of_squares) {
                                            if MagicSquare::new(a, b, c, d, e, f, g, h, i).sums_are_equal() {
                                                println!("{:?}", MagicSquare::new(a, b, c, d, e, f, g, h, i));
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
