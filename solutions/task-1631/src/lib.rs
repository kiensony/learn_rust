// rust-track: handwritten task-1631
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1631;

pub fn clamp_property_trace(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        input
            .iter()
            .map(|&x| u32::from(x.clamp(4, 16) != x))
            .collect()
    };
    samples
}
