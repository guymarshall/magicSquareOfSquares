mod combiner;
mod square;

use combiner::parallel_merge_all;
use square::sums_are_valid;
use std::collections::HashMap;
use std::process::exit;
use std::sync::{Mutex, MutexGuard};
use std::time::Instant;
use sysinfo::System;

use rayon::{current_num_threads, prelude::*};

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

fn main() {
    let start_time: Instant = Instant::now();

    const SQUARE_NUMBERS: [usize; LIMIT] = generate_square_numbers();

    let mut system: System = System::new_all();
    system.refresh_memory();

    let total_memory_bytes: u64 = system.total_memory();
    let memory_budget: usize = (total_memory_bytes / 5) as usize;

    let entry_size: usize = std::mem::size_of::<(usize, usize)>();
    let chunk_size_total: usize = memory_budget / entry_size;
    let chunk_size: usize = chunk_size_total / current_num_threads();

    let totals_and_counts_per_thread: Vec<Mutex<HashMap<usize, usize>>> =
        (0..current_num_threads())
            .map(|_| Mutex::new(HashMap::new()))
            .collect();

    println!("Getting most frequent total...");
    SQUARE_NUMBERS
        .par_iter()
        .enumerate()
        .for_each(|(i, first): (usize, &usize)| {
            let totals =
                SQUARE_NUMBERS
                    .iter()
                    .enumerate()
                    .flat_map(|(j, second): (usize, &usize)| {
                        SQUARE_NUMBERS
                            .iter()
                            .skip(j + 1)
                            .map(move |third: &usize| first + second + third)
                    });

            let thread_id: usize = rayon::current_thread_index().unwrap();
            let mut thread_totals_and_counts: MutexGuard<'_, HashMap<usize, usize>> =
                totals_and_counts_per_thread[thread_id]
                    .lock()
                    .expect("Failed to lock mutex");

            totals.for_each(|total: usize| {
                *thread_totals_and_counts.entry(total).or_insert(0) += 1;
            });

            if i % chunk_size == 0 && i != LIMIT - 1 {
                let thread_totals_and_counts_cloned: HashMap<usize, usize> =
                    thread_totals_and_counts.clone();
                let mut thread_sorted: Vec<(&usize, &usize)> =
                    thread_totals_and_counts_cloned.iter().collect();
                thread_sorted.sort_by(|a: &(&usize, &usize), b: &(&usize, &usize)| b.1.cmp(a.1));
                let top_10: HashMap<&usize, &usize> = thread_sorted
                    .into_iter()
                    .take(10)
                    .map(|(total, count): (&usize, &usize)| (total, count))
                    .collect();
                thread_totals_and_counts.clear();
                top_10
                    .into_iter()
                    .for_each(|(total, count): (&usize, &usize)| {
                        thread_totals_and_counts.insert(*total, *count);
                    });
            }
        });

    let merged_totals: HashMap<usize, usize> = parallel_merge_all(totals_and_counts_per_thread);

    let most_frequent_total: usize = *merged_totals
        .par_iter()
        .max_by_key(|&(_, count): &(&usize, &usize)| count)
        .unwrap()
        .0;
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

                        for i in 0..9 {
                            for j in (i + 1)..9 {
                                if merged_rows[i] == merged_rows[j] {
                                    continue;
                                }
                            }
                        }

                        println!("{:?}", merged_rows);
                        exit(0);
                    }
                }
            });
    }

    let end_time: Instant = Instant::now();

    println!("Elapsed time: {:?}", end_time - start_time);
}
