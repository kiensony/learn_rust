// rust-track: handwritten task-2227
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2227;

pub fn borrowed_scalars_distinct(input: &str) -> Vec<u32> {
    let mut samples: Vec<u32> = { input.char_indices().map(|(i, _)| i as u32).collect() };
    samples.sort_unstable();
    samples.dedup();
    samples
}
