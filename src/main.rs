use itertools::Itertools;

fn generate_square_numbers(count: i32) -> Vec<i32> {
    (0..=count).map(|number| number * number).collect()
}

fn sums_are_equal(a: &i32, b: &i32, c: &i32, d: &i32, e: &i32, f: &i32, g: &i32, h: &i32, i: &i32) -> bool {
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

    let square_numbers: Vec<i32> = generate_square_numbers(LIMIT);

    let permutations = square_numbers.iter().permutations(9);

    permutations.for_each(|permutation| {
        if !(permutation[0] == permutation[1]
            || permutation[0] == permutation[2]
            || permutation[0] == permutation[3]
            || permutation[0] == permutation[4]
            || permutation[0] == permutation[5]
            || permutation[0] == permutation[6]
            || permutation[0] == permutation[7]
            || permutation[0] == permutation[8]
            || permutation[1] == permutation[2]
            || permutation[1] == permutation[3]
            || permutation[1] == permutation[4]
            || permutation[1] == permutation[5]
            || permutation[1] == permutation[6]
            || permutation[1] == permutation[7]
            || permutation[1] == permutation[8]
            || permutation[2] == permutation[3]
            || permutation[2] == permutation[4]
            || permutation[2] == permutation[5]
            || permutation[2] == permutation[6]
            || permutation[2] == permutation[7]
            || permutation[2] == permutation[8]
            || permutation[3] == permutation[4]
            || permutation[3] == permutation[5]
            || permutation[3] == permutation[6]
            || permutation[3] == permutation[7]
            || permutation[3] == permutation[8]
            || permutation[4] == permutation[5]
            || permutation[4] == permutation[6]
            || permutation[4] == permutation[7]
            || permutation[4] == permutation[8]
            || permutation[5] == permutation[6]
            || permutation[5] == permutation[7]
            || permutation[5] == permutation[8]
            || permutation[6] == permutation[7]
            || permutation[6] == permutation[8]
            || permutation[7] == permutation[8]) {
            if sums_are_equal(permutation[0], permutation[1], permutation[2], permutation[3], permutation[4], permutation[5], permutation[6], permutation[7], permutation[8]) {
                println!("{:?}", [permutation[0], permutation[1], permutation[2], permutation[3], permutation[4], permutation[5], permutation[6], permutation[7], permutation[8]]);
            }
        }
    });
}
