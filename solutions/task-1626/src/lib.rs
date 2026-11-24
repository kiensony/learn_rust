// rust-track: handwritten task-1626
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1626;

pub fn round_trip_property_descending_steps(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        input
            .iter()
            .map(|&x| u32::from(x.wrapping_mul(2) / 2 != x))
            .collect()
    };
    samples.windows(2).filter(|p| p[1] < p[0]).count() as u64
}
