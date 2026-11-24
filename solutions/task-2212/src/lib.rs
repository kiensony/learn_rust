// rust-track: handwritten task-2212
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2212;

pub fn borrowed_offsets_total(input: &str) -> u64 {
    let samples: Vec<u32> = { input.match_indices('\n').map(|(i, _)| i as u32).collect() };
    samples.into_iter().map(u64::from).sum()
}
