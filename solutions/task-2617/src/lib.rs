// rust-track: handwritten task-2617
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2617;

pub fn embedded_flags_distinct(input: &[u8]) -> Vec<u32> {
    let mut samples = observe_embedded_flags(input);
    samples.sort_unstable();
    samples.dedup();
    samples
}

fn observe_embedded_flags(input: &[u8]) -> Vec<u32> {
    input
        .iter()
        .map(|&x| u32::from(track_device::convert(x)))
        .collect()
}
