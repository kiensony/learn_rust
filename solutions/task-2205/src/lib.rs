// rust-track: handwritten task-2205
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2205;

pub fn borrowed_prefixes_changes(input: &str) -> Vec<u32> {
    let mut samples: Vec<u32> = {
        input
            .split_ascii_whitespace()
            .map(|w| w.len() as u32)
            .collect()
    };
    samples.dedup();
    samples
}
