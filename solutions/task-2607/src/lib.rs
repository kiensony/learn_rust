// rust-track: handwritten task-2607
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2607;

pub fn embedded_checksum_distinct(input: &[u8]) -> Vec<u32> {
    let mut samples = observe_embedded_checksum(input);
    samples.sort_unstable();
    samples.dedup();
    samples
}

fn observe_embedded_checksum(input: &[u8]) -> Vec<u32> {
    input
        .iter()
        .map(|&x| u32::from(track_device::convert(x)))
        .collect()
}
