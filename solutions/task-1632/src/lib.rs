// rust-track: handwritten task-1632
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1632;

pub fn clamp_property_total(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        input
            .iter()
            .map(|&x| u32::from(x.clamp(4, 16) != x))
            .collect()
    };
    samples.into_iter().map(u64::from).sum()
}
