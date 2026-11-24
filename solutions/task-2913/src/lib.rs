// rust-track: handwritten task-2913
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2913;

pub fn latency_improvement_bounds(input: &[u8]) -> Option<(u32, u32)> {
    let samples: Vec<u32> = {
        input
            .chunks_exact(2)
            .map(|p| u32::from(p[0].saturating_sub(p[1])))
            .collect()
    };
    samples
        .iter()
        .min()
        .copied()
        .zip(samples.iter().max().copied())
}
