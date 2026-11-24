// rust-track: handwritten task-2207
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2207;

pub fn borrowed_prefixes_distinct(input: &str) -> Vec<u32> {
    let mut samples = observe_borrowed_prefixes(input);
    samples.sort_unstable();
    samples.dedup();
    samples
}

fn observe_borrowed_prefixes(input: &str) -> Vec<u32> {
    input
        .split_ascii_whitespace()
        .map(|w| w.len() as u32)
        .collect()
}
