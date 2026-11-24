// rust-track: handwritten task-2225
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2225;

pub fn borrowed_scalars_changes(input: &str) -> Vec<u32> {
    let mut samples: Vec<u32> = { input.char_indices().map(|(i, _)| i as u32).collect() };
    samples.dedup();
    samples
}
