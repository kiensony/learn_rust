// rust-track: handwritten task-1641
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1641;

pub fn shift_property_trace(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        input
            .iter()
            .map(|&x| u32::from((x >> 1) << 1 != x))
            .collect()
    };
    samples
}
