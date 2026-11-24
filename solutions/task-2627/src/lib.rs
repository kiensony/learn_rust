// rust-track: handwritten task-2627
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2627;

pub fn embedded_parity_distinct(input: &[u8]) -> Vec<u32> {
    let mut samples: Vec<u32> = {
        input
            .iter()
            .map(|&x| u32::from(track_device::convert(x)))
            .collect()
    };
    samples.sort_unstable();
    samples.dedup();
    samples
}
