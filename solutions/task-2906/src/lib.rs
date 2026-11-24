// rust-track: handwritten task-2906
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2906;

pub fn latency_increases_descending_steps(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        input
            .chunks_exact(2)
            .map(|p| u32::from(p[1].saturating_sub(p[0])))
            .collect()
    };
    samples.windows(2).filter(|p| p[1] < p[0]).count() as u64
}
