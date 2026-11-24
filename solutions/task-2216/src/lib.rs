// rust-track: handwritten task-2216
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2216;

pub fn borrowed_offsets_descending_steps(input: &str) -> u64 {
    let samples: Vec<u32> = { input.match_indices('\n').map(|(i, _)| i as u32).collect() };
    samples.windows(2).filter(|p| p[1] < p[0]).count() as u64
}
