// rust-track: handwritten task-2213
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2213;

pub fn borrowed_offsets_bounds(input: &str) -> Option<(u32, u32)> {
    let samples: Vec<u32> = { input.match_indices('\n').map(|(i, _)| i as u32).collect() };
    samples
        .iter()
        .min()
        .copied()
        .zip(samples.iter().max().copied())
}
