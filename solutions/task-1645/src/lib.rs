// rust-track: handwritten task-1645
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1645;

pub fn shift_property_changes(input: &[u8]) -> Vec<u32> {
    let mut samples: Vec<u32> = {
        input
            .iter()
            .map(|&x| u32::from((x >> 1) << 1 != x))
            .collect()
    };
    samples.dedup();
    samples
}
