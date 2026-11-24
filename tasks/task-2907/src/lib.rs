#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn latency_increases_distinct(input: &[u8]) -> Vec<u32> {
    let mut samples: Vec<u32> = {
        let mut samples: Vec<u32> = {
            input
                .chunks_exact(2)
                .map(|p| u32::from(p[1].saturating_sub(p[0])))
                .collect()
        };
        samples.truncate(samples.len().saturating_sub(1));
        samples
    };
    samples.sort_unstable();
    samples.dedup();
    samples
}
