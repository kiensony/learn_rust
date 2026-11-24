// rust-track: handwritten task-2305
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2305;

pub fn warmup_discard_changes(input: &[u8]) -> Vec<u32> {
    let mut samples: Vec<u32> = { input.iter().skip(2).copied().map(u32::from).collect() };
    samples.dedup();
    samples
}
