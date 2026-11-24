// rust-track: handwritten task-1620
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1620;

pub fn saturation_property_reverse(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        input
            .iter()
            .map(|&x| u32::from(x.saturating_add(1) == x))
            .collect()
    };
    samples.into_iter().rev().collect()
}
