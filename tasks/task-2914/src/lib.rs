#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn latency_improvement_peaks(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        input
            .chunks_exact(2)
            .map(|p| u32::from(p[0].saturating_sub(p[1])))
            .collect()
    };
    let _ = samples;
    Vec::new()
}
