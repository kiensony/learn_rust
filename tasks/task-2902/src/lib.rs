#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn latency_increases_total(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        input
            .chunks_exact(2)
            .map(|p| u32::from(p[1].saturating_sub(p[0])))
            .collect()
    };
    let _ = samples;
    0
}
