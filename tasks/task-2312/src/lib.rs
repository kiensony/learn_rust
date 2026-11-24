#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn paired_speedup_total(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        input
            .chunks_exact(2)
            .map(|p| u32::from(p[1].saturating_sub(p[0])))
            .collect()
    };
    samples.into_iter().map(u64::from).sum()
}
