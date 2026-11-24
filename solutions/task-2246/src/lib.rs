// rust-track: handwritten task-2246
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2246;

pub fn borrowed_prefixes_threshold_crossing(input: &str, limit: u32) -> Option<(usize, u32)> {
    let observations: Vec<u32> = {
        input
            .split_ascii_whitespace()
            .map(|w| w.len() as u32)
            .collect()
    };
    observations
        .into_iter()
        .enumerate()
        .find(|&(_, value)| value >= limit)
}
