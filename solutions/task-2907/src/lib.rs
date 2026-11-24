// rust-track: handwritten task-2907
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2907;

pub fn latency_increases_distinct(input: &[u8]) -> Vec<u32> {
    let mut samples = observe_latency_increases(input);
    samples.sort_unstable();
    samples.dedup();
    samples
}

fn observe_latency_increases(input: &[u8]) -> Vec<u32> {
    input
        .chunks_exact(2)
        .map(|p| u32::from(p[1].saturating_sub(p[0])))
        .collect()
}
