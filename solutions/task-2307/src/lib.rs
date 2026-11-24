// rust-track: handwritten task-2307
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2307;

pub fn warmup_discard_distinct(input: &[u8]) -> Vec<u32> {
    let mut samples = observe_warmup_discard(input);
    samples.sort_unstable();
    samples.dedup();
    samples
}

fn observe_warmup_discard(input: &[u8]) -> Vec<u32> {
    input.iter().skip(2).copied().map(u32::from).collect()
}
