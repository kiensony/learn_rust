// rust-track: handwritten task-2143
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2143;

pub fn first_occurrences_bounds(input: &[u8]) -> Option<(u32, u32)> {
    let samples: Vec<u32> = {
        let mut seen = std::collections::HashSet::new();
        input
            .iter()
            .filter(|&&x| seen.insert(x))
            .map(|&x| u32::from(x))
            .collect()
    };
    samples
        .iter()
        .min()
        .copied()
        .zip(samples.iter().max().copied())
}
