mod square;

use std::process;
use std::time::Instant;

use crate::square::{numbers_are_unique, sums_are_equal};

fn generate_square_numbers<const COUNT: usize>() -> [usize; COUNT] {
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

    const LIMIT: usize = 50;
    let square_numbers: [usize; LIMIT] = generate_square_numbers();

    square_numbers.iter().for_each(|first| {
        square_numbers.iter().for_each(|second| {
            square_numbers.iter().for_each(|third| {
                square_numbers.iter().for_each(|fourth| {
                    square_numbers.iter().for_each(|fifth| {
                        square_numbers.iter().for_each(|sixth| {
                            square_numbers.iter().for_each(|seventh| {
                                square_numbers.iter().for_each(|eighth| {
                                    square_numbers.iter().for_each(|ninth| {
                                        if sums_are_equal([*first, *second, *third, *fourth, *fifth, *sixth, *seventh, *eighth, *ninth]) && numbers_are_unique([*first, *second, *third, *fourth, *fifth, *sixth, *seventh, *eighth, *ninth]) {
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
        println!("{} / {}", (*first as f32).sqrt(), LIMIT);
    });

    let end_time: Instant = Instant::now();

    println!("Elapsed time: {:?}", end_time - start_time);
}
