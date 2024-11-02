mod database;
mod square;

use itertools::Itertools;
use square::{numbers_are_unique, sums_are_valid};
use std::process::exit;
use std::time::Instant;
use std::{collections::HashMap, io::Write};

use database::{clear_totals, delete_db, get_total_with_highest_count, init, insert};
use rayon::prelude::*;

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

fn get_most_frequent_total(square_numbers: &[usize; LIMIT]) -> Option<usize> {
    let mut totals_and_counts: HashMap<usize, usize> = HashMap::with_capacity(LIMIT);

    for (i, first) in square_numbers.iter().enumerate() {
        square_numbers
            .iter()
            .combinations(2)
            .for_each(|permutation: Vec<&usize>| {
                *totals_and_counts
                    .entry(first + permutation[0] + permutation[1])
                    .or_insert(0) += 1;
            });

        if i % 100 == 0 {
            insert("db.sqlite", &totals_and_counts).unwrap();
            totals_and_counts.clear();
        }

        let percentage_progress: f32 = ((i + 1) as f32 / square_numbers.len() as f32) * 100.0;
        print!("\rGetting most frequent total: {:.1}%", percentage_progress);
        std::io::stdout().flush().unwrap();
    }

    if !totals_and_counts.is_empty() {
        insert("db.sqlite", &totals_and_counts).unwrap();
    }

    println!();

    get_total_with_highest_count("db.sqlite").unwrap()
}

fn main() {
    let start_time: Instant = Instant::now();

    init("db.sqlite").unwrap();
    clear_totals("db.sqlite").unwrap();

    const SQUARE_NUMBERS: [usize; LIMIT] = generate_square_numbers();
    let most_frequent_total: usize = get_most_frequent_total(&SQUARE_NUMBERS).unwrap();
    println!("The most frequent total is {}", most_frequent_total);

    delete_db("db.sqlite").unwrap();

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
