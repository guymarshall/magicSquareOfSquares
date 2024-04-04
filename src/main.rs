use rayon::prelude::*;
use std::io::{stdout, StdoutLock, Write};
use std::process;
use std::time::Instant;

fn numbers_are_unique(numbers: [&usize; 9]) -> bool {
    if numbers[0] == numbers[1] || numbers[0] == numbers[2] || numbers[0] == numbers[3] || numbers[0] == numbers[4] || numbers[0] == numbers[5] || numbers[0] == numbers[6] || numbers[0] == numbers[7] || numbers[0] == numbers[8] {
        return false;
    }

    if numbers[1] == numbers[2] || numbers[1] == numbers[3] || numbers[1] == numbers[4] || numbers[1] == numbers[5] || numbers[1] == numbers[6] || numbers[1] == numbers[7] || numbers[1] == numbers[8] {
        return false;
    }

    if numbers[2] == numbers[3] || numbers[2] == numbers[4] || numbers[2] == numbers[5] || numbers[2] == numbers[6] || numbers[2] == numbers[7] || numbers[2] == numbers[8] {
        return false;
    }

    if numbers[3] == numbers[4] || numbers[3] == numbers[5] || numbers[3] == numbers[6] || numbers[3] == numbers[7] || numbers[3] == numbers[8] {
        return false;
    }

    if numbers[4] == numbers[5] || numbers[4] == numbers[6] || numbers[4] == numbers[7] || numbers[4] == numbers[8] {
        return false;
    }

    if numbers[5] == numbers[6] || numbers[5] == numbers[7] || numbers[5] == numbers[8] {
        return false;
    }

    if numbers[6] == numbers[7] || numbers[6] == numbers[8] {
        return false;
    }

    numbers[7] != numbers[8]
}

fn sums_are_equal(numbers: [&usize; 9]) -> bool {
    if (numbers[0] + numbers[1] + numbers[2]) != (numbers[3] + numbers[4] + numbers[5])
        || (numbers[3] + numbers[4] + numbers[5]) != (numbers[6] + numbers[7] + numbers[8])
    {
        return false;
    }

    if (numbers[6] + numbers[7] + numbers[8]) != (numbers[0] + numbers[3] + numbers[6])
        || (numbers[0] + numbers[3] + numbers[6]) != (numbers[1] + numbers[4] + numbers[7])
        || (numbers[1] + numbers[4] + numbers[7]) != (numbers[2] + numbers[5] + numbers[8])
    {
        return false;
    }

    (numbers[2] + numbers[5] + numbers[8]) == (numbers[0] + numbers[4] + numbers[8])
        && (numbers[0] + numbers[4] + numbers[8]) == (numbers[6] + numbers[4] + numbers[2])
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
    let start_time: Instant = Instant::now();

    const LIMIT: usize = 10;
    const LIMIT_SQUARED: usize = LIMIT * LIMIT;
    const SQUARE_NUMBERS: [usize; LIMIT] = generate_square_numbers();

    let mut lock: StdoutLock = stdout().lock();

    SQUARE_NUMBERS.iter().for_each(|a| {
        SQUARE_NUMBERS.par_iter().for_each(|b| {
            SQUARE_NUMBERS.iter().for_each(|c| {
                SQUARE_NUMBERS.iter().for_each(|d| {
                    SQUARE_NUMBERS.iter().for_each(|e| {
                        SQUARE_NUMBERS.iter().for_each(|f| {
                            SQUARE_NUMBERS.iter().for_each(|g| {
                                SQUARE_NUMBERS.iter().for_each(|h| {
                                    SQUARE_NUMBERS.iter().for_each(|i| {
                                        if sums_are_equal([a, b, c, d, e, f, g, h, i])
                                            && numbers_are_unique([a, b, c, d, e, f, g, h, i])
                                        {
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
        writeln!(lock, "{} / {}", a, LIMIT_SQUARED).unwrap();
    });

    let end_time: Instant = Instant::now();

    println!("Elapsed time: {:?}", end_time - start_time);
}

/*
***** PERFORMANCE TESTS (release running limit of 10 on an AMD Ryzen 5 4600H) *****

using array of numbers - MEAN 64.6621474
    67.969869
    64.63394
    51.490696
    71.216327
    67.999905

new numbers_are_unique implementation - MEAN 47.5416206
    43.752971
    45.510972
    49.633446
    50.446042
    48.364672

swap order of sums_are_equal() and numbers_are_unique() checks - MEAN 8.2879586
    9.506104
    9.403209
    8.990686
    5.00276
    8.537034
*/