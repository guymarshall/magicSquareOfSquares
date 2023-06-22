use crate::magic_square::MagicSquare;

mod magic_square;

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
