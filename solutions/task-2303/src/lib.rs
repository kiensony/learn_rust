// rust-track: handwritten task-2303
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2303;

pub fn warmup_discard_bounds(input: &[u8]) -> Option<(u32, u32)> {
    let samples: Vec<u32> = { input.iter().skip(2).copied().map(u32::from).collect() };
    samples
        .iter()
        .min()
        .copied()
        .zip(samples.iter().max().copied())
}
