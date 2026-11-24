// rust-track: handwritten task-2230
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2230;

pub fn borrowed_scalars_reverse(input: &str) -> Vec<u32> {
    let samples: Vec<u32> = { input.char_indices().map(|(i, _)| i as u32).collect() };
    samples.into_iter().rev().collect()
}
