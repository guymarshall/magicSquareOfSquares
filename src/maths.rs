use rayon::current_num_threads;
use sysinfo::System;

#[inline(always)]
pub fn get_chunk_size() -> usize {
    let mut system: System = System::new_all();
    system.refresh_memory();

    let total_memory_bytes: u64 = system.total_memory();
    let memory_budget: usize = (total_memory_bytes / 5) as usize;

    let entry_size: usize = std::mem::size_of::<(usize, usize)>();
    let chunk_size_total: usize = memory_budget / entry_size;
    chunk_size_total / current_num_threads()
}
