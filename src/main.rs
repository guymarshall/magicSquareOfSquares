use crate::magic_square::MagicSquare;
use rayon::prelude::*;

mod magic_square;

fn numbers_are_unique(number1: i32, number2: i32, number3: i32, number4: i32, number5: i32, number6: i32, number7: i32, number8: i32, number9: i32) -> bool {
    !(number1 == number2
        || number1 == number3
        || number1 == number4
        || number1 == number5
        || number1 == number6
        || number1 == number7
        || number1 == number8
        || number1 == number9
        || number2 == number3
        || number2 == number4
        || number2 == number5
        || number2 == number6
        || number2 == number7
        || number2 == number8
        || number2 == number9
        || number3 == number4
        || number3 == number5
        || number3 == number6
        || number3 == number7
        || number3 == number8
        || number3 == number9
        || number4 == number5
        || number4 == number6
        || number4 == number7
        || number4 == number8
        || number4 == number9
        || number5 == number6
        || number5 == number7
        || number5 == number8
        || number5 == number9
        || number6 == number7
        || number6 == number8
        || number6 == number9
        || number7 == number8
        || number7 == number9
        || number8 == number9)
}

fn main() {
    const LIMIT: i32 = 40;

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
                                    });
                                });
                            });
                        });
                    });
                });
            });
            println!("{} / {}", b, LIMIT);
        });
    });
}
