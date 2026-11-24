#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn paired_speedup_changes(input: &[u8]) -> Vec<u32> {
    let mut samples: Vec<u32> = {
        input
            .chunks_exact(2)
            .map(|p| u32::from(p[1].saturating_sub(p[0])))
            .collect()
    };
    samples.dedup();
    samples
}
