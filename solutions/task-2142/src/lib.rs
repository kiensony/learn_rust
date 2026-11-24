// rust-track: handwritten task-2142
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2142;

pub fn first_occurrences_total(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        let mut seen = std::collections::HashSet::new();
        input
            .iter()
            .filter(|&&x| seen.insert(x))
            .map(|&x| u32::from(x))
            .collect()
    };
    samples.into_iter().map(u64::from).sum()
}
