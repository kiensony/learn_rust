// rust-track: handwritten task-1642
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1642;

pub fn shift_property_total(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        input
            .iter()
            .map(|&x| u32::from((x >> 1) << 1 != x))
            .collect()
    };
    samples.into_iter().map(u64::from).sum()
}
