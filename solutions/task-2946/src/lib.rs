// rust-track: handwritten task-2946
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2946;

pub fn latency_increases_threshold_crossing(input: &[u8], limit: u32) -> Option<(usize, u32)> {
    let observations: Vec<u32> = {
        input
            .chunks_exact(2)
            .map(|p| u32::from(p[1].saturating_sub(p[0])))
            .collect()
    };
    observations
        .into_iter()
        .enumerate()
        .find(|&(_, value)| value >= limit)
}
