// rust-track: handwritten task-2203
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2203;

pub fn borrowed_prefixes_bounds(input: &str) -> Option<(u32, u32)> {
    let samples: Vec<u32> = {
        input
            .split_ascii_whitespace()
            .map(|w| w.len() as u32)
            .collect()
    };
    samples
        .iter()
        .min()
        .copied()
        .zip(samples.iter().max().copied())
}
