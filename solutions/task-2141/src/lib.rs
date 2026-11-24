// rust-track: handwritten task-2141
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2141;

pub fn first_occurrences_trace(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        let mut seen = std::collections::HashSet::new();
        input
            .iter()
            .filter(|&&x| seen.insert(x))
            .map(|&x| u32::from(x))
            .collect()
    };
    samples
}
