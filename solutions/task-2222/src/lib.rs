// rust-track: handwritten task-2222
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2222;

pub fn borrowed_scalars_total(input: &str) -> u64 {
    let samples: Vec<u32> = { input.char_indices().map(|(i, _)| i as u32).collect() };
    samples.into_iter().map(u64::from).sum()
}
