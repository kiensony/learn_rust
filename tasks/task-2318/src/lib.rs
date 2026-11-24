#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn paired_speedup_steps(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        input
            .chunks_exact(2)
            .map(|p| u32::from(p[1].saturating_sub(p[0])))
            .collect()
    };
    samples.windows(2).map(|p| p[0].abs_diff(p[1])).collect()
}
