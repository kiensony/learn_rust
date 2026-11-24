// rust-track: handwritten task-2602
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2602;

pub fn embedded_checksum_total(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        input
            .iter()
            .map(|&x| u32::from(track_device::convert(x)))
            .collect()
    };
    samples.into_iter().map(u64::from).sum()
}
