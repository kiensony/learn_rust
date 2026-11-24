// rust-track: handwritten task-1611
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1611;

pub fn saturation_property_trace(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        input
            .iter()
            .map(|&x| u32::from(x.saturating_add(1) == x))
            .collect()
    };
    samples
}
