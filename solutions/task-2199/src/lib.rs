// rust-track: handwritten task-2199
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2199;

pub fn early_exit_probes_frequent_readings(input: &[u8], keep: usize) -> Vec<u32> {
    let observations: Vec<u32> = {
        input
            .iter()
            .enumerate()
            .map(|(i, &x)| input[..i].iter().position(|&y| y > x).map_or(i, |j| j + 1) as u32)
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
