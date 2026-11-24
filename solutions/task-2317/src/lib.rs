// rust-track: handwritten task-2317
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2317;

pub fn paired_speedup_distinct(input: &[u8]) -> Vec<u32> {
    let mut samples = observe_paired_speedup(input);
    samples.sort_unstable();
    samples.dedup();
    samples
}

fn observe_paired_speedup(input: &[u8]) -> Vec<u32> {
    input
        .chunks_exact(2)
        .map(|p| u32::from(p[0].saturating_sub(p[1])))
        .collect()
}
