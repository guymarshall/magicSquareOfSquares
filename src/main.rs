use crate::magic_square::MagicSquare;
use rayon::prelude::*;

mod magic_square;

fn numbers_are_unique(number1: i32, number2: i32, number3: i32, number4: i32, number5: i32, number6: i32, number7: i32, number8: i32, number9: i32) -> bool {
    let mask: u8 = (1 << number1) | (1 << number2) | (1 << number3) | (1 << number4) | (1 << number5) | (1 << number6) | (1 << number7) | (1 << number8) | (1 << number9);
    mask.count_ones() == 9
}

fn main() {
    const LIMIT: i32 = 10;

    (0..LIMIT).into_par_iter().for_each(|a| {
        (0..LIMIT).for_each(|b| {
            (0..LIMIT).for_each(|c| {
                (0..LIMIT).for_each(|d| {
                    (0..LIMIT).for_each(|e| {
                        (0..LIMIT).for_each(|f| {
                            (0..LIMIT).for_each(|g| {
                                (0..LIMIT).for_each(|h| {
                                    (0..LIMIT).for_each(|i| {
                                        let numbers_are_unique: bool = numbers_are_unique(a, b, c, d, e, f, g, h, i);

                                        if numbers_are_unique {
                                            let square: MagicSquare = MagicSquare::new(a, b, c, d, e, f, g, h, i);
                                            let is_magic: bool = square.sums_are_equal();

                                            if is_magic {
                                                println!("{:?}", square);
                                            }
                                        }
                                    })
                                })
                            })
                        })
                    })
                })
            })
        })
    })
}
