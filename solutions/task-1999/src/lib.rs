// rust-track: handwritten task-1999
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1999;

pub fn atomic_record_high_frequent_readings(input: &[u8], keep: usize) -> Vec<u32> {
    let observations: Vec<u32> = {
        use std::sync::atomic::{AtomicU32, Ordering};
        let cell = AtomicU32::new(0);
        input
            .iter()
            .map(|&x| cell.fetch_max(u32::from(x), Ordering::Relaxed))
            .collect()
    };
    let mut counts = std::collections::BTreeMap::new();
    for (index, value) in observations.into_iter().enumerate() {
        let (count, _) = counts.entry(value).or_insert((0usize, index));
        *count += 1;
    }
    let mut entries: Vec<_> = counts.into_iter().collect();
    entries.sort_unstable_by_key(|(_, (count, first))| (std::cmp::Reverse(*count), *first));
    entries
        .into_iter()
        .take(keep)
        .map(|(value, _)| value)
        .collect()
}
