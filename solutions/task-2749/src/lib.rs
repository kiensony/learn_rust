// rust-track: handwritten task-2749
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2749;

pub fn cli_rust_sources_frequent_readings(input: &str, keep: usize) -> Vec<u32> {
    let observations: Vec<u32> = {
        input
            .split_ascii_whitespace()
            .map(std::path::Path::new)
            .enumerate()
            .filter(|(_, p)| p.extension().is_some_and(|e| e == "rs"))
            .map(|(i, _)| i as u32)
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
