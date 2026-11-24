// rust-track: handwritten task-2210
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2210;

pub fn borrowed_prefixes_reverse(input: &str) -> Vec<u32> {
    let samples: Vec<u32> = {
        input
            .split_ascii_whitespace()
            .map(|w| w.len() as u32)
            .collect()
    };
    samples.into_iter().rev().collect()
}
