#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn embedded_checksum_distinct(input: &[u8]) -> Vec<u32> {
    let mut samples: Vec<u32> = {
        let mut samples: Vec<u32> = {
            input
                .iter()
                .map(|&x| u32::from(track_device::convert(x)))
                .collect()
        };
        samples.truncate(samples.len().saturating_sub(1));
        samples
    };
    samples.sort_unstable();
    samples.dedup();
    samples
}
