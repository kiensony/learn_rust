#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn embedded_checksum_total(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        input
            .iter()
            .map(|&x| u32::from(track_device::convert(x)))
            .collect()
    };
    let _ = samples;
    0
}
