// rust-track: handwritten task-1627
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1627;

pub fn round_trip_property_distinct(input: &[u8]) -> Vec<u32> {
    let mut samples: Vec<u32> = {
        input
            .iter()
            .map(|&x| u32::from(x.wrapping_mul(2) / 2 != x))
            .collect()
    };
    samples.sort_unstable();
    samples.dedup();
    samples
}
