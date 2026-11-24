// rust-track: handwritten task-2610
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2610;

pub fn embedded_checksum_reverse(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        input
            .iter()
            .map(|&x| u32::from(track_device::convert(x)))
            .collect()
    };
    samples.into_iter().rev().collect()
}
