use rayon::prelude::*;

fn numbers_are_unique(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32, g: i32, h: i32, i: i32) -> bool {
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

fn sums_are_equal(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32, g: i32, h: i32, i: i32) -> bool {
    let top_row_sum: i32 = a + b + c;
    let middle_row_sum: i32 = d + e + f;
    let bottom_row_sum: i32 = g + h + i;

    if top_row_sum != middle_row_sum || middle_row_sum != bottom_row_sum {
        return false;
    }

    let left_column_sum: i32 = a + d + g;
    let middle_column_sum: i32 = b + e + h;
    let right_column_sum: i32 = c + f + i;

    if bottom_row_sum != left_column_sum || left_column_sum != middle_column_sum || middle_column_sum != right_column_sum {
        return false;
    }

    let nw_se_sum: i32 = a + e + i;
    let sw_ne_sum: i32 = g + e + c;

    right_column_sum == nw_se_sum && nw_se_sum == sw_ne_sum
}

fn main() {
    const LIMIT: i32 = 80;

    (0..LIMIT).for_each(|a| {
        (0..LIMIT).into_par_iter().for_each(|b| {
            (0..LIMIT).for_each(|c| {
                (0..LIMIT).for_each(|d| {
                    (0..LIMIT).for_each(|e| {
                        (0..LIMIT).for_each(|f| {
                            (0..LIMIT).for_each(|g| {
                                (0..LIMIT).for_each(|h| {
                                    (0..LIMIT).for_each(|i| {
                                        if numbers_are_unique(a, b, c, d, e, f, g, h, i) {
                                            if sums_are_equal(a * a, b * b, c * c, d * d, e * e, f * f, g * g, h * h, i * i) {
                                                println!("{:?}", [a * a, b * b, c * c, d * d, e * e, f * f, g * g, h * h, i * i]);
                                            }
                                        }
                                    });
                                });
                            });
                        });
                    });
                });
            });
        });
        println!("{} / {}", a, LIMIT);
    });
}
