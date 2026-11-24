// rust-track: handwritten task-2201
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2201;

pub fn borrowed_prefixes_trace(input: &str) -> Vec<u32> {
    let samples: Vec<u32> = {
        input
            .split_ascii_whitespace()
            .map(|w| w.len() as u32)
            .collect()
    };
    samples
}
