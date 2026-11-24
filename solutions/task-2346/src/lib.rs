// rust-track: handwritten task-2346
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2346;

pub fn warmup_discard_threshold_crossing(input: &[u8], limit: u32) -> Option<(usize, u32)> {
    let observations: Vec<u32> = { input.iter().skip(2).copied().map(u32::from).collect() };
    observations
        .into_iter()
        .enumerate()
        .find(|&(_, value)| value >= limit)
}
