use rayon::prelude::*;
use std::{collections::HashMap, sync::Mutex};

fn merge_totals(
    mut map1: HashMap<usize, usize>,
    map2: HashMap<usize, usize>,
) -> HashMap<usize, usize> {
    map2.into_iter().for_each(|(key, value2): (usize, usize)| {
        map1.entry(key)
            .and_modify(|value1: &mut usize| *value1 += value2)
            .or_insert(value2);
    });
    map1
}

pub(crate) fn parallel_merge_all(maps: Vec<Mutex<HashMap<usize, usize>>>) -> HashMap<usize, usize> {
    maps.into_par_iter()
        .map(|mutex_map: Mutex<HashMap<usize, usize>>| {
            mutex_map
                .into_inner()
                .expect("Failed to lock the mutex during parallel merge")
        })
        .reduce(HashMap::new, merge_totals)
}
