// rust-track: handwritten task-2910
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2910;

pub fn latency_increases_reverse(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        input
            .chunks_exact(2)
            .map(|p| u32::from(p[1].saturating_sub(p[0])))
            .collect()
    };
    samples.into_iter().rev().collect()
}
