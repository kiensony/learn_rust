// rust-track: handwritten task-1637
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1637;

pub fn clamp_property_distinct(input: &[u8]) -> Vec<u32> {
    let mut samples: Vec<u32> = {
        input
            .iter()
            .map(|&x| u32::from(x.clamp(4, 16) != x))
            .collect()
    };
    samples.sort_unstable();
    samples.dedup();
    samples
}
