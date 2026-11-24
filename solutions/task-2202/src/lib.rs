// rust-track: handwritten task-2202
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2202;

pub fn borrowed_prefixes_total(input: &str) -> u64 {
    let samples: Vec<u32> = {
        input
            .split_ascii_whitespace()
            .map(|w| w.len() as u32)
            .collect()
    };
    samples.into_iter().map(u64::from).sum()
}
