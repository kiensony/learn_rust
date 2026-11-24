// rust-track: handwritten task-1546
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1546;

pub fn packet_high_nibble_threshold_crossing(input: &[u8], limit: u32) -> Option<(usize, u32)> {
    let observations: Vec<u32> = { input.iter().map(|&x| u32::from(wire::decode(x))).collect() };
    observations
        .into_iter()
        .enumerate()
        .find(|&(_, value)| value >= limit)
}
