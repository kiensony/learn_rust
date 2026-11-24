// rust-track: handwritten task-2912
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2912;

pub fn latency_improvement_total(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        input
            .chunks_exact(2)
            .map(|p| u32::from(p[0].saturating_sub(p[1])))
            .collect()
    };
    samples.into_iter().map(u64::from).sum()
}
