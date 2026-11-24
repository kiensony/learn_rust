// rust-track: handwritten task-1646
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1646;

pub fn parity_property_threshold_crossing(input: &[u8], limit: u32) -> Option<(usize, u32)> {
    let observations: Vec<u32> = { input.iter().map(|&x| u32::from(x % 2 != (x & 1))).collect() };
    observations
        .into_iter()
        .enumerate()
        .find(|&(_, value)| value >= limit)
}
