#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn warmup_discard_distinct(input: &[u8]) -> Vec<u32> {
    let mut samples: Vec<u32> = {
        let mut samples: Vec<u32> = { input.iter().skip(2).copied().map(u32::from).collect() };
        samples.truncate(samples.len().saturating_sub(1));
        samples
    };
    samples.sort_unstable();
    samples.dedup();
    samples
}
