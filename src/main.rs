use std::ops::Range;
use std::process;
use rayon::prelude::*;

fn numbers_are_unique(a: u32, b: u32, c: u32, d: u32, e: u32, f: u32, g: u32, h: u32, i: u32) -> bool {
    !(a == b
        || a == c
        || a == d
        || a == e
        || a == f
        || a == g
        || a == h
        || a == i
        || b == c
        || b == d
        || b == e
        || b == f
        || b == g
        || b == h
        || b == i
        || c == d
        || c == e
        || c == f
        || c == g
        || c == h
        || c == i
        || d == e
        || d == f
        || d == g
        || d == h
        || d == i
        || e == f
        || e == g
        || e == h
        || e == i
        || f == g
        || f == h
        || f == i
        || g == h
        || g == i
        || h == i)
}

fn sums_are_equal(a: u32, b: u32, c: u32, d: u32, e: u32, f: u32, g: u32, h: u32, i: u32) -> bool {
    let top_row_sum: u32 = a + b + c;
    let middle_row_sum: u32 = d + e + f;
    let bottom_row_sum: u32 = g + h + i;

    if top_row_sum != middle_row_sum || middle_row_sum != bottom_row_sum {
        return false;
    }

    let left_column_sum: u32 = a + d + g;
    let middle_column_sum: u32 = b + e + h;
    let right_column_sum: u32 = c + f + i;

    if bottom_row_sum != left_column_sum || left_column_sum != middle_column_sum || middle_column_sum != right_column_sum {
        return false;
    }

    let nw_se_sum: u32 = a + e + i;
    let sw_ne_sum: u32 = g + e + c;

    right_column_sum == nw_se_sum && nw_se_sum == sw_ne_sum
}

fn main() {
    const LIMIT: u32 = 60;
    const NUMBER_ITERATOR: Range<u32> = 0..LIMIT;

    NUMBER_ITERATOR.for_each(|a| {
        NUMBER_ITERATOR.into_par_iter().for_each(|b| {
            NUMBER_ITERATOR.for_each(|c| {
                NUMBER_ITERATOR.for_each(|d| {
                    NUMBER_ITERATOR.for_each(|e| {
                        NUMBER_ITERATOR.for_each(|f| {
                            NUMBER_ITERATOR.for_each(|g| {
                                NUMBER_ITERATOR.for_each(|h| {
                                    NUMBER_ITERATOR.for_each(|i| {
                                        if numbers_are_unique(a, b, c, d, e, f, g, h, i) && sums_are_equal(a * a, b * b, c * c, d * d, e * e, f * f, g * g, h * h, i * i) {
                                            println!("{:?}", [a * a, b * b, c * c, d * d, e * e, f * f, g * g, h * h, i * i]);
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
        println!("{} / {}", a + 1, LIMIT);
    });
}