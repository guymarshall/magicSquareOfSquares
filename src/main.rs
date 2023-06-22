use crate::magic_square::MagicSquare;

mod magic_square;

fn numbers_are_unique(number1: i32, number2: i32, number3: i32, number4: i32, number5: i32, number6: i32, number7: i32, number8: i32, number9: i32) -> bool {
    let mut count = [0; 10];

    let numbers = [number1, number2, number3, number4, number5, number6, number7, number8, number9];

    for &number in &numbers {
        count[number as usize] += 1;
        if count[number as usize] > 1 {
            return false;
        }
    }

    true
}

fn main() {
    const LIMIT: i32 = 2;
    for a in 0..LIMIT {
        for b in 0..LIMIT {
            for c in 0..LIMIT {
                for d in 0..LIMIT {
                    for e in 0..LIMIT {
                        for f in 0..LIMIT {
                            for g in 0..LIMIT {
                                for h in 0..LIMIT {
                                    for i in 0..LIMIT {
                                        let numbers_are_unique: bool = numbers_are_unique(a, b, c, d, e, f, g, h, i);

                                        if numbers_are_unique {
                                            let square: MagicSquare = MagicSquare::new(a, b, c, d, e, f, g, h, i);
                                            let is_magic: bool = square.sums_are_equal();

                                            if is_magic {
                                                println!("{:?}", square);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
