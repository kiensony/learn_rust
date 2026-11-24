// rust-track: handwritten task-2226
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2226;

pub fn borrowed_scalars_descending_steps(input: &str) -> u64 {
    let samples: Vec<u32> = { input.char_indices().map(|(i, _)| i as u32).collect() };
    samples.windows(2).filter(|p| p[1] < p[0]).count() as u64
}
