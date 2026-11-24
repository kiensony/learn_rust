// rust-track: handwritten task-2218
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2218;

pub fn borrowed_offsets_steps(input: &str) -> Vec<u32> {
    let samples: Vec<u32> = { input.match_indices('\n').map(|(i, _)| i as u32).collect() };
    samples.windows(2).map(|p| p[0].abs_diff(p[1])).collect()
}
