// rust-track: handwritten task-2249
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2249;

pub fn zero_copy_lines_frequent_readings(input: &str, keep: usize) -> Vec<u32> {
    let observations: Vec<u32> = { input.lines().map(|l| l.len() as u32).collect() };
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
