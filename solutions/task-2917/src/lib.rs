// rust-track: handwritten task-2917
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2917;

pub fn latency_improvement_distinct(input: &[u8]) -> Vec<u32> {
    let mut samples = observe_latency_improvement(input);
    samples.sort_unstable();
    samples.dedup();
    samples
}

fn observe_latency_improvement(input: &[u8]) -> Vec<u32> {
    input
        .chunks_exact(2)
        .map(|p| u32::from(p[0].saturating_sub(p[1])))
        .collect()
}
