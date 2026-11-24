// rust-track: handwritten task-2625
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2625;

pub fn embedded_parity_changes(input: &[u8]) -> Vec<u32> {
    let mut samples: Vec<u32> = {
        input
            .iter()
            .map(|&x| u32::from(track_device::convert(x)))
            .collect()
    };
    samples.dedup();
    samples
}
