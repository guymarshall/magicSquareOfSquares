mod allocator;
mod square;

use allocator::preallocate_hashmap;
use square::{numbers_are_unique, sums_are_valid};
use std::process::exit;
use std::time::Instant;
use std::{collections::HashMap, io::Write};

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

fn generate_totals(square_numbers: &[usize], first: &usize) -> Vec<usize> {
    let mut totals: Vec<usize> = Vec::new();

    for i in 0..LIMIT {
        for j in i + 1..LIMIT {
            let sum: usize = first + square_numbers[i] + square_numbers[j];
            totals.push(sum);
        }
    }

    totals
}

fn get_most_frequent_total(square_numbers: &[usize; LIMIT]) -> usize {
    let (mut totals_and_counts, chunk_size): (HashMap<usize, usize>, usize) = preallocate_hashmap();

    for (i, first) in square_numbers.iter().enumerate() {
        let totals: Vec<usize> = generate_totals(square_numbers, first);

        totals.into_iter().for_each(|total: usize| {
            *totals_and_counts.entry(total).or_insert(0) += 1;
        });

        if i % chunk_size == 0 && i != LIMIT - 1 {
            let mut sorted_totals_and_counts: Vec<(&usize, &usize)> =
                totals_and_counts.par_iter().collect();

            sorted_totals_and_counts
                .sort_by(|a: &(&usize, &usize), b: &(&usize, &usize)| b.1.cmp(a.1));

            let top_10: HashMap<usize, usize> = sorted_totals_and_counts
                .iter()
                .take(10)
                .map(|(total, count): &(&usize, &usize)| (**total, **count))
                .collect();

            totals_and_counts.clear();

            top_10.iter().for_each(|(total, count): (&usize, &usize)| {
                totals_and_counts.insert(*total, *count);
            });
        }

        let percentage_progress: f32 = ((i + 1) as f32 / square_numbers.len() as f32) * 100.0;
        print!("\rGetting most frequent total: {:.1}%", percentage_progress);
        std::io::stdout().flush().unwrap();
    }

    println!();

    *totals_and_counts
        .par_iter()
        .max_by_key(|&(_, count): &(&usize, &usize)| count)
        .unwrap()
        .0
}

fn main() {
    let start_time: Instant = Instant::now();

    const SQUARE_NUMBERS: [usize; LIMIT] = generate_square_numbers();
    let most_frequent_total: usize = get_most_frequent_total(&SQUARE_NUMBERS);
    println!("The most frequent total is {}", most_frequent_total);

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
    for top_row in triplets_that_make_total.iter() {
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
