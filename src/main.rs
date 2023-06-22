use crate::magic_square::MagicSquare;

mod magic_square;

fn numbers_are_unique(n1: i32, n2: i32, n3: i32, n4: i32, n5: i32, n6: i32, n7: i32, n8: i32, n9: i32) -> bool {
    let mut count = [0; 10];

    let numbers = [n1, n2, n3, n4, n5, n6, n7, n8, n9];

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
