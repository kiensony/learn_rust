// rust-track: handwritten task-1799
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1799;

pub fn token_multiplier_frequent_readings(input: &[u8], keep: usize) -> Vec<u32> {
    let observations: Vec<u32> = {
        let units: u32 = arity::arity!(unit unit unit unit);
        input.iter().map(|&x| u32::from(x) * units).collect()
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
