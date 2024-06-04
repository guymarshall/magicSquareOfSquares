mod square;

use std::process;
use std::time::Instant;

use rayon::prelude::*;
use crate::square::Square;

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
    const LIMIT_SQUARED: usize = LIMIT * LIMIT;
    const SQUARE_NUMBERS: [usize; LIMIT] = generate_square_numbers();

    SQUARE_NUMBERS.iter().for_each(|first| {
        SQUARE_NUMBERS.iter().for_each(|second| {
            SQUARE_NUMBERS.par_iter().for_each(|third| {
                SQUARE_NUMBERS.iter().for_each(|fourth| {
                    SQUARE_NUMBERS.iter().for_each(|fifth| {
                        SQUARE_NUMBERS.iter().for_each(|sixth| {
                            SQUARE_NUMBERS.iter().for_each(|seventh| {
                                SQUARE_NUMBERS.iter().for_each(|eighth| {
                                    SQUARE_NUMBERS.iter().for_each(|ninth| {
                                        let square: Square = Square {first, second, third, fourth, fifth, sixth, seventh, eighth, ninth};
                                        if square.sums_are_equal() && square.numbers_are_unique() {
                                            println!("{:?}", [first, second, third, fourth, fifth, sixth, seventh, eighth, ninth]);
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
        println!("{} / {}", first, LIMIT_SQUARED);
    });

    let end_time: Instant = Instant::now();

    println!("Elapsed time: {:?}", end_time - start_time);
}
