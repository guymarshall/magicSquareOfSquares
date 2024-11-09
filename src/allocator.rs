use std::collections::HashMap;
use sysinfo::System;

pub(crate) fn preallocate_hashmap() -> (HashMap<usize, usize>, usize) {
    let mut system: System = System::new_all();
    system.refresh_memory();

    let total_memory_bytes: u64 = system.total_memory();
    let memory_budget: usize = (total_memory_bytes / 5) as usize;

    let entry_size: usize = std::mem::size_of::<(usize, usize)>();
    let max_entries: usize = memory_budget / entry_size;

    (HashMap::with_capacity(max_entries), max_entries)
}
