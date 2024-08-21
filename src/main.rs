mod square;

use rayon::prelude::*;
use std::process;
use std::time::Instant;

use crate::square::{numbers_are_unique, sums_are_equal};

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
    let start_time: Instant = Instant::now();

    const LIMIT: usize = 100;
    const SQUARE_NUMBERS: [usize; LIMIT] = generate_square_numbers();

    SQUARE_NUMBERS.iter().for_each(|first: &usize| {
        SQUARE_NUMBERS.par_iter().for_each(|second: &usize| {
            if second != first {
                SQUARE_NUMBERS.iter().for_each(|third: &usize| {
                    SQUARE_NUMBERS.iter().for_each(|fourth: &usize| {
                        SQUARE_NUMBERS.iter().for_each(|fifth: &usize| {
                            SQUARE_NUMBERS.iter().for_each(|sixth: &usize| {
                                SQUARE_NUMBERS.iter().for_each(|seventh: &usize| {
                                    SQUARE_NUMBERS.iter().for_each(|eighth: &usize| {
                                        SQUARE_NUMBERS.iter().for_each(|ninth: &usize| {
                                            if sums_are_equal(first, second, third, fourth, fifth, sixth, seventh, eighth, ninth) && numbers_are_unique(first, second, third, fourth, fifth, sixth, seventh, eighth, ninth) {
                                                println!("\n\n{}, {}, {}\n{}, {}, {}\n{}, {}, {}\n\n", first, second, third, fourth, fifth, sixth, seventh, eighth, ninth);
                                                process::exit(0);
                                            }
                                        });
                                    });
                                });
                            });
                        });
                    });
                });
            }
        });
        println!("{} / {}", (*first as f32).sqrt(), LIMIT);
    });

    let end_time: Instant = Instant::now();

    println!("Elapsed time: {:?}", end_time - start_time);
}
