// rust-track: handwritten task-2605
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2605;

pub fn embedded_checksum_changes(input: &[u8]) -> Vec<u32> {
    let mut samples: Vec<u32> = {
        input
            .iter()
            .map(|&x| u32::from(track_device::convert(x)))
            .collect()
    };
    samples.dedup();
    samples
}
