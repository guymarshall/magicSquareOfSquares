mod database;

use std::collections::HashMap;
use std::process::exit;
use std::time::Instant;

use database::{clear_totals, delete_db, get_total_with_highest_count, init, insert};
use dotenv::var;
use rayon::prelude::*;
use rusqlite::Connection;

const LIMIT: usize = 1000;

const fn generate_square_numbers<const LIMIT: usize>() -> [usize; LIMIT] {
    let mut numbers: [usize; LIMIT] = [0usize; LIMIT];

    let mut counter: usize = 0;
    while counter < LIMIT {
        numbers[counter] = (counter + 1) * (counter + 1);
        counter += 1;
    }

    numbers
}

fn get_most_frequent_total(
    connection: &mut Connection,
    square_numbers: &[usize; LIMIT],
) -> Option<usize> {
    square_numbers.iter().for_each(|first: &usize| {
        let totals: Vec<usize> = square_numbers
            .par_iter()
            .flat_map(|second: &usize| {
                square_numbers
                    .par_iter()
                    .map(move |third: &usize| first + second + third)
                    .collect::<Vec<_>>()
            })
            .collect();

        let totals_and_counts: HashMap<usize, usize> = totals
            .par_iter()
            .fold(
                HashMap::new,
                |mut map: HashMap<usize, usize>, total: &usize| {
                    *map.entry(*total).or_insert(0) += 1;
                    map
                },
            )
            .reduce(
                HashMap::new,
                |mut map1: HashMap<usize, usize>, map2: HashMap<usize, usize>| {
                    map2.iter().for_each(|(key, count)| {
                        *map1.entry(*key).or_insert(0) += count;
                    });
                    map1
                },
            );

        insert(connection, &totals_and_counts).unwrap();

        let current: f32 = (*first as f32).sqrt();
        let percentage_progress: f32 = (current / LIMIT as f32) * 100.0;
        println!("Getting most frequent total: {:.1}%", percentage_progress);
    });

    get_total_with_highest_count(connection).unwrap()
}

#[inline(always)]
fn numbers_are_unique(numbers: &[usize; 9]) -> bool {
    for i in 0..9 {
        for j in (i + 1)..9 {
            if numbers[i] == numbers[j] {
                return false;
            }
        }
    }
    true
}

#[inline(always)]
fn sums_are_valid(
    top_row: &[usize; 3],
    middle_row: &[usize; 3],
    bottom_row: &[usize; 3],
    most_frequent_total: usize,
) -> bool {
    // don't need to check row sums as they are already correct

    // columns
    if top_row[0] + middle_row[0] + bottom_row[0] != most_frequent_total {
        return false;
    }
    if top_row[1] + middle_row[1] + bottom_row[1] != most_frequent_total {
        return false;
    }
    if top_row[2] + middle_row[2] + bottom_row[2] != most_frequent_total {
        return false;
    }

    // diagonals
    if top_row[0] + middle_row[1] + bottom_row[2] != most_frequent_total {
        return false;
    }
    if top_row[2] + middle_row[1] + bottom_row[0] != most_frequent_total {
        return false;
    }

    true
}

fn main() {
    let start_time: Instant = Instant::now();

    let db_path: String = var("SQLITE_DB_PATH").expect("SQLITE_DB_PATH not set");
    let mut connection: Connection = Connection::open(db_path.as_str()).unwrap();

    init(&connection).unwrap();
    clear_totals(&connection).unwrap();

    const SQUARE_NUMBERS: [usize; LIMIT] = generate_square_numbers();
    let most_frequent_total: usize =
        get_most_frequent_total(&mut connection, &SQUARE_NUMBERS).unwrap();
    println!("The most frequent total is {}", most_frequent_total);

    delete_db(connection, &db_path).unwrap();

    println!("Calculating triplets_that_make_total");
    let triplets_that_make_total: Vec<[usize; 3]> = SQUARE_NUMBERS
        .par_iter()
        .flat_map(|&first| {
            SQUARE_NUMBERS.par_iter().flat_map(move |&second| {
                SQUARE_NUMBERS.par_iter().filter_map(move |&third| {
                    let total: usize = first + second + third;
                    if total == most_frequent_total {
                        Some([first, second, third])
                    } else {
                        None
                    }
                })
            })
        })
        .collect();

    println!("Checking triples");
    for top_row in triplets_that_make_total.clone().iter() {
        triplets_that_make_total
            .par_iter()
            .for_each(|middle_row: &[usize; 3]| {
                for bottom_row in &triplets_that_make_total {
                    if sums_are_valid(top_row, middle_row, bottom_row, most_frequent_total) {
                        let merged_rows: [usize; 9] = [
                            top_row[0],
                            top_row[1],
                            top_row[2],
                            middle_row[0],
                            middle_row[1],
                            middle_row[2],
                            bottom_row[0],
                            bottom_row[1],
                            bottom_row[2],
                        ];

                        if numbers_are_unique(&merged_rows) {
                            println!("{:?}", merged_rows);
                            exit(0);
                        }
                    }
                }
            });
    }

    let end_time: Instant = Instant::now();

    println!("Elapsed time: {:?}", end_time - start_time);
}
