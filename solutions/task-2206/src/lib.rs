// rust-track: handwritten task-2206
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2206;

pub fn borrowed_prefixes_descending_steps(input: &str) -> u64 {
    let samples: Vec<u32> = {
        input
            .split_ascii_whitespace()
            .map(|w| w.len() as u32)
            .collect()
    };
    samples.windows(2).filter(|p| p[1] < p[0]).count() as u64
}
